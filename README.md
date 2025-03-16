<h1 align="center">LessOJ</h1>

<div align="center">
  <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/TwinklerG/Less-OJ">
  <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/TwinklerG/Less-OJ">
  <img alt="GitHub stars" src="https://img.shields.io/github/stars/TwinklerG/Less-OJ">
</div>

Another Online Judge System based on Next.js, Rocket.rs and Docker.

## Dependencies

- [Cargo](https://github.com/rust-lang/cargo)
- [Node.js](https://nodejs.org/)
- [Docker](https://www.docker.com/)

## Development

1. start the worker service in `worker` folder which needs docker

    ```shell
    systemctl start docker
    cargo run
    ```

2. start the webapp service in `webapp` folder

    ```shell
    npm install
    npm run dev
    ```

> If you want to customize sandbox in docker, you can develop in `sandbox` folder, which is also an ordinary rust project.