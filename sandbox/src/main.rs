use clap::Parser;
use nix::libc::{
    chdir, gid_t, kill, pid_t, rlimit, rusage, setgid, setrlimit, setuid, strsignal, uid_t, wait4,
    RLIMIT_AS, RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_NPROC, RLIMIT_STACK, SIGKILL, SIGXCPU, SIGXFSZ,
    WEXITSTATUS, WIFEXITED, WTERMSIG,
};
use nix::unistd::{fork, ForkResult};
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::prelude::CommandExt;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SANDBOX_UID: usize = 1111;
const SANDBOX_GID: usize = 1111;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = 1)]
    time_limit: u64,
    #[clap(long, default_value_t = 1)]
    time_reserved: u64,
    #[clap(short, long, default_value_t = 256000)]
    memory_limit: u64,
    #[clap(long, default_value_t = 256000)]
    memory_reserved: u64,
    #[clap(short, long, default_value = "./script")]
    command: String,
    #[clap(long, default_value = "in")]
    file_stdin: String,
    #[clap(long, default_value = "out")]
    file_stdout: String,
    #[clap(long, default_value = "err")]
    file_stderr: String,
    #[clap(long, default_value = "result")]
    file_result: String,
    #[clap(long, default_value_t = 0)]
    large_stack: u64,
    #[clap(long, default_value_t = 0)]
    output_limit: u64,
    #[clap(long, default_value_t = 0)]
    process_limit: u64,
}

#[derive(Serialize, Deserialize)]
enum ExitState {
    Success,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    OtherError,
}

#[derive(Serialize, Deserialize)]
pub struct SandboxResult {
    state: ExitState,

    stdout: String,
    stderr: String,
    time: u64,   // ms
    memory: u64, // kB
}

impl SandboxResult {
    fn new() -> SandboxResult {
        SandboxResult {
            state: ExitState::OtherError,
            stdout: "".to_string(),
            stderr: "".to_string(),
            time: 0,
            memory: 0,
        }
    }
}

fn main() {
    let args = Args::parse();

    // make sure relative folder is 777
    unsafe {
        chdir(CString::new("/sandbox").unwrap().into_raw());
        setuid(SANDBOX_UID as uid_t);
        setgid(SANDBOX_GID as gid_t);
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Entering Parent Process");
            let mut sandbox_result: SandboxResult = SandboxResult::new();
            let time_limit_exceeded_killed = Arc::new(Mutex::new(false));
            {
                let time_limit_exceeded_killed = Arc::clone(&time_limit_exceeded_killed);
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(args.time_limit + args.time_reserved));
                    unsafe {
                        kill(pid_t::from(child), SIGKILL);
                    }
                    *time_limit_exceeded_killed.lock().unwrap() = true;
                });
            }
            let mut usage: rusage = unsafe { std::mem::zeroed() };
            let mut status = 0;
            if unsafe { wait4(pid_t::from(child), &mut status, 0, &mut usage) == -1 } {
                println!("RuntimeError: Wait4() == -1");
                return;
            }
            if WIFEXITED(status) {
                // Not signaled - exited normally
                if WEXITSTATUS(status) != 0 {
                    println!(
                        "Runtime Error WIFEXITED - WEXITSTATUS() = {}",
                        WEXITSTATUS(status)
                    );
                    sandbox_result.state = ExitState::RuntimeError;
                } else {
                    println!(
                        "Exited Normally WIFEXITED - WEXITSTATUS() = {}",
                        WEXITSTATUS(status)
                    );
                    sandbox_result.state = ExitState::Success;
                }
            } else {
                // signaled
                let sig = WTERMSIG(status);
                if sig == SIGXCPU
                    || usage.ru_utime.tv_usec > args.time_limit as i64
                    || *time_limit_exceeded_killed.lock().unwrap()
                {
                    println!(
                        "Time Limit Exceeded WEXITSTATUS() = {}, WTERMSIG() = {} ({})",
                        WEXITSTATUS(status),
                        sig,
                        unsafe { *strsignal(sig) }
                    );
                    sandbox_result.state = ExitState::TimeLimitExceeded;
                } else if sig == SIGXFSZ {
                    println!(
                        "Output Limit Exceeded WEXITSTATUS() = {}, WTERMSIG() = {} ({})",
                        WEXITSTATUS(status),
                        sig,
                        unsafe { *strsignal(sig) }
                    );
                    sandbox_result.state = ExitState::OtherError;
                } else if usage.ru_maxrss > args.memory_limit as i64 {
                    println!(
                        "Memory Limit Exceeded WEXITSTATUS() = {}, WTERMSIG() = {} ({})",
                        WEXITSTATUS(status),
                        sig,
                        unsafe { *strsignal(sig) }
                    );
                    sandbox_result.state = ExitState::MemoryLimitExceeded;
                } else {
                    println!(
                        "Runtime Error WEXITSTATUS() = {}, WTERMSIG() = {} ({})",
                        WEXITSTATUS(status),
                        sig,
                        unsafe { *strsignal(sig) }
                    );
                    sandbox_result.state = ExitState::RuntimeError;
                }
            }
            sandbox_result.time =
                ((usage.ru_utime.tv_sec * 1000000 + usage.ru_utime.tv_usec) / 1000) as u64; // ms
            sandbox_result.memory = usage.ru_maxrss as u64; // kB
            let mut stdout = std::fs::File::open("out").unwrap();
            let mut stderr = std::fs::File::open("err").unwrap();
            stdout.read_to_string(&mut sandbox_result.stdout).unwrap();
            stderr.read_to_string(&mut sandbox_result.stderr).unwrap();
            println!("memory_usage = {}KB", usage.ru_maxrss);
            println!(
                "time_usage = {}ms",
                (usage.ru_utime.tv_sec * 1000000 + usage.ru_utime.tv_usec) / 1000
            );
            std::fs::write(
                "result.toml",
                toml::to_string(&sandbox_result).unwrap().as_bytes(),
            )
            .unwrap();
        }
        Ok(ForkResult::Child) => {
            println!("Entering Child Process");

            println!("time limit: {}s", args.time_limit);
            println!("memory_limit: {}KB", args.memory_limit);

            let time_limit = rlimit {
                rlim_cur: args.time_limit,
                rlim_max: args.time_limit,
            };
            unsafe {
                setrlimit(RLIMIT_CPU, &time_limit);
            }

            let memo_limit = rlimit {
                rlim_cur: args.memory_limit * 1024,
                rlim_max: args.memory_limit * 1024,
            };
            unsafe {
                setrlimit(RLIMIT_AS, &memo_limit);
            }
            if args.large_stack != 0 {
                unsafe {
                    setrlimit(RLIMIT_STACK, &memo_limit);
                }
            }

            if args.output_limit != 0 {
                let output_limit = rlimit {
                    rlim_cur: args.output_limit,
                    rlim_max: args.output_limit,
                };
                unsafe {
                    setrlimit(RLIMIT_FSIZE, &output_limit);
                }
            }

            if args.process_limit != 0 {
                let process_limit = rlimit {
                    rlim_cur: args.process_limit + 1,
                    rlim_max: args.process_limit + 1,
                };
                unsafe {
                    setrlimit(RLIMIT_NPROC, &process_limit);
                }
            }

            let stdout_file =
                File::create(args.file_stdout).unwrap_or(File::create("/dev/null").unwrap());
            let stdin_file =
                File::open(args.file_stdin).unwrap_or(File::create("/dev/null").unwrap());
            let stderr_file =
                File::create(args.file_stderr).unwrap_or(File::create("/dev/null").unwrap());

            let mut command = Command::new(&args.command);
            let _ = command
                .stdout(unsafe { std::process::Stdio::from_raw_fd(stdout_file.into_raw_fd()) })
                .stdin(unsafe { std::process::Stdio::from_raw_fd(stdin_file.into_raw_fd()) })
                .stderr(unsafe { std::process::Stdio::from_raw_fd(stderr_file.into_raw_fd()) })
                .exec();
        }
        Err(_) => {}
    }
}
