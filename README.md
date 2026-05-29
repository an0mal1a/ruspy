# ruspy

A Remote Access Tool (RAT) built from scratch in Rust — developed purely as a learning project.

> **Disclaimer:** This project is for educational purposes only. Use only in controlled environments with explicit authorization. The author is not responsible for any misuse.

## Purpose

The goal of this project is to learn Rust by building something non-trivial that touches:

- TCP sockets and network programming
- Asynchronous I/O
- File system interaction
- OS/system interaction
- Client/server architecture

No AI-generated code — everything written by hand to maximize learning.

## Architecture

The project is a Cargo workspace with two crates:

```
ruspy/
├── server/   # C2 — listens for connections, sends instructions
└── client/   # Agent — connects to the server, executes instructions
```

The **server** binds to a port, accepts a connection, and lets the operator type commands interactively via stdin.  
The **client** connects to the server and dispatches each received instruction.

## Current state

- [x] TCP connection between server and client
- [x] Basic instruction dispatch loop
- [x] Graceful disconnection (`q` command)
- [ ] Change instruct send of string to enum
- [ ] Shell command execution
- [ ] File upload / download
- [ ] Persistence
- [ ] Multi-client support
- [ ] Secure communication (AES256)
- [ ] GUI (maybe)
- [ ] And more

## Usage

**Run the server first:**

```bash
cargo run -p server
```

**Then run the client:**

```bash
cargo run -p client
```

The client connects to `127.0.0.1:1337` by default. From the server you can type instructions — currently `hello` and `q` (quit) are recognized.

## Requirements

- Rust (stable toolchain) — [rustup.rs](https://rustup.rs)

## License

MIT
