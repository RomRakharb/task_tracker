# Task Tracker

*this is my first project in [roadmap.sh's Backend Roadmap](https://roadmap.sh/projects/task-tracker) series*

## How to use

1. Setup your dev environment: choose 1
    - Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - use devcontainer(from .devcontainer.json)
    - or use nix flake (I use NixOS btw)

2. run it
    - `cargo run add "finish your project"`
    - `cargo run update 1 "go to sleep"`
    - `cargo run delete 1`
    - `cargo run mark-done 2`
    - `cargo run list in-progress`
