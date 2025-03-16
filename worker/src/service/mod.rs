use std::collections::HashMap;
use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

const SANDBOX_FOLDER: &str = "sandbox";

#[derive(Serialize, Deserialize)]
enum ExitState {
    Success,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    CompileError,
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

pub struct Config {
    image: String,
    pub env: HashMap<String, String>,
    input: String,

    pub time_limit: u64,
    time_limit_reserved: u64,
    pub memory_limit: u64,
    pub memory_reserved: u64,
    command: String,
    // file_stdin: String,
    file_stdout: String,
    file_stderr: String,
    file_result: String,
    large_stack: u64,
    output_limit: u64,
    process_limit: u64,
}

impl Config {
    pub fn new(image: String, input: String) -> Config {
        Config {
            image,
            env: HashMap::new(),
            input,

            time_limit: 1,
            time_limit_reserved: 1,
            memory_limit: 256000,
            memory_reserved: 256000,
            command: "./script".to_string(),
            // file_stdin: "in".to_string(),
            file_stdout: "out".to_string(),
            file_stderr: "err".to_string(),
            file_result: "result".to_string(),
            large_stack: 0,
            output_limit: 0,
            process_limit: 0,
        }
    }
}

pub fn sandbox_service(
    src: &str,
    src_name: &str,
    script: Vec<&str>,
    option: &Config,
) -> SandboxResult {
    let perm = fs::Permissions::from_mode(0o777);
    if !Path::new(SANDBOX_FOLDER).exists() {
        fs::create_dir(SANDBOX_FOLDER).expect("Unable to create SANDBOX folder");
        fs::set_permissions(SANDBOX_FOLDER, perm.clone())
            .expect("Failed to set permission for folder");
    }
    fs::write(format!("./sandbox/{}", src_name), src).expect("Unable to write main.cpp");
    fs::set_permissions(format!("./sandbox/{}", src_name), perm.clone())
        .expect("Unable to set permission for main.cpp");
    fs::write("./sandbox/in", &option.input).expect("Unable to write in");
    for s in script {
        println!("{}", s);
        fs::write("./sandbox/script", s).expect("Unable to write script");
        fs::set_permissions("./sandbox/script", perm.clone())
            .expect("Unable to set permission for script");
        let mut command = Command::new("docker");
        command.arg("run").arg("--rm");
        // .arg("-it") lead to error "The input device is not a TTY"
        for (k, v) in &option.env {
            println!("-e {}={}", k, v);
            command.arg("-e");
            command.arg(format!("{}={}", k, v));
        }
        let _ = command.arg("-v").arg("./sandbox:/sandbox")
            .arg(option.image.as_str())
            .arg("bash")
            .arg("-c")
            .arg(format!(
                "/sandbox/sandbox -t {} --time-reserved {} -m {} --memory-reserved {} -c {} --file-stdin {} --file-stdout {} --file-stderr {} --file-result {} --large-stack {} --output-limit {} --process-limit {}",
                option.time_limit, option.time_limit_reserved, option.memory_limit, option.memory_reserved,option.command,  "in", option.file_stdout, option.file_stderr,option.file_result, option.large_stack, option.output_limit, option.process_limit
            ))
            // .arg("/sandbox/sandbox")
            .stdout(Stdio::piped()) // Capture stdout
            .stderr(Stdio::piped()) // Capture stderr
            .output()
            .unwrap(); // Execute
    }
    let entries = fs::read_dir("sandbox").unwrap();
    let ret =
        toml::from_str(&String::from_utf8(fs::read("./sandbox/result.toml").unwrap()).unwrap())
            .unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() && path.file_name() != Some(std::ffi::OsStr::new("sandbox")) {
            fs::remove_file(path).unwrap();
        } else if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        }
    }
    ret
}
