# Rust-SQL

[![Docker Image CI lint](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/lint.yml/badge.svg)](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/lint.yml)

[![Docker Image CI Main](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/main.yml/badge.svg)](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/main.yml)

[![Docker Image CI release](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/make_release_binary.yml/badge.svg)](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/make_release_binary.yml)

[![Docker Image CI test](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/test.yml/badge.svg)](https://github.com/Nathan-Bush46/IDS706-rust-IP2/actions/workflows/test.yml)

# Rust SQLite Demo

This project demonstrates basic CRUD operations using Rust and SQLite.

## Dependencies

- Rust
- SQLite

## Running and Set

1. See Docker set up to install needed libraries. (This installs needed Dependencies for you)
3. Run `cargo build` to build the project. (or make)
4. Run `cargo run` to execute the program. (or use make all to build, lint, and run)
5. make release will make optimized binary and run it once
 
## Set up instructions using VS code + Docker: 
### Docker
1. For Windows, Mac, and maybe Linux, you download Docker Desktop. links can be found [here](https://docs.docker.com/engine/install/). Follow set up instructions and start the application.

2. In vs code add Dev Containers and Docker extensions 

3. clone repo, restart vs code, and open repo in vs code.

4. should see a pop up to (re)open in devcontainer. Click it and let it run. It takes a bit of time for the first run but is much faster after that. hit enter to install rust when promoted, restart terminal. Done.

