# rush 🐚

A Unix shell built from scratch in Rust using nothing but the standard library. No external crates, no frameworks — just `std::process`, `std::io`, and a loop.

Built as a learning project to understand how shells actually work: process spawning, file descriptors, pipes, and built-in commands.

---

## Demo

```
rush> help
Built-in commands: cd, help, exit

rush> ls -la
total 48
drwxr-xr-x 4 lewis lewis 4096 Jun  1 10:00 .
drwxr-xr-x 8 lewis lewis 4096 Jun  1 09:00 ..
-rw-r--r-- 1 lewis lewis  198 Jun  1 10:00 Cargo.toml

rush> cd src
rush> pwd
/home/lewis/rust/rush/src

rush> ls | grep .rs
main.rs

rush> echo hello world > out.txt
rush> cat out.txt
hello world

rush> exit
```

---

## Features

- **REPL loop** — read, evaluate, print, loop
- **Run any program** — `ls`, `echo`, `cat`, `grep` — anything on your PATH
- **Built-in commands** — `cd`, `help`, `exit`
- **Output redirection** — `cmd > file.txt`
- **Pipes** — `cmd1 | cmd2`

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)

### Build and run

```bash
git clone https://github.com/yourusername/rush
cd rush
cargo run
```

---

## Usage

### Run any command
```
rush> ls -la
rush> echo hello world
rush> pwd
```

### Change directory
```
rush> cd src
rush> pwd
/home/lewis/rust/rush/src
rush> cd ..
```

### Output redirection
```
rush> ls > out.txt
rush> cat out.txt
```

### Pipes
```
rush> ls | grep .rs
rush> cat Cargo.toml | grep name
```

### Exit
```
rush> exit
```

---

## How It Works

### REPL Loop
The shell runs in an infinite loop — print a prompt, read a line, execute it, repeat. This is the foundation of every shell ever built.

### Command parsing
```rust
let mut parts = line.split_whitespace();
let cmd = parts.next().unwrap_or("");
let args: Vec<&str> = parts.collect();
```
Splits `ls -la` into command `ls` and args `["-la"]`.

### Process spawning
```rust
Command::new(cmd)
    .args(args)
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
```
Asks the OS to start a new child process. `.wait()` blocks until it finishes.

### Built-in commands
`cd` cannot be a child process — changing directory in a subprocess has no effect on the shell itself. So built-ins run directly inside the shell process using `std::env::set_current_dir`.

### Output redirection
When `>` is found in the args, the file is created and passed to `.stdout(file)` on the `Command`. The OS connects the process's stdout directly to that file.

### Pipes
```rust
let left = Command::new(cmd)
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

Command::new(right_cmd)
    .stdin(left.stdout.unwrap())
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
```
The left command's stdout is captured with `Stdio::piped()` and fed directly as stdin to the right command.

---

## Key Rust Concepts

| Concept | Where used |
|---|---|
| `std::process::Command` | spawning every external command |
| `Stdio::piped()` | capturing output for pipes |
| `std::env::set_current_dir` | built-in `cd` |
| `std::fs::File::create` | output redirection |
| `Iterator::position` | finding `>` and `\|` in args |
| `split_whitespace` | parsing command input |

---

## Ideas for Extending

- [ ] **Input redirection** — `cmd < file.txt`
- [ ] **Append redirection** — `cmd >> file.txt`
- [ ] **Multiple pipes** — `cmd1 | cmd2 | cmd3`
- [ ] **Environment variables** — `$HOME`, `$PATH`
- [ ] **Command history** — arrow keys to scroll through previous commands
- [ ] **Tab completion** — complete file and command names
- [ ] **Background jobs** — `cmd &`
- [ ] **Signal handling** — proper `Ctrl+C` handling
- [ ] **Script execution** — run a `.sh` file of commands

---

## What I Learned

- How the OS actually runs programs — fork, exec, wait
- Why `cd` must be a shell built-in and can't be a subprocess
- How file descriptors work — stdout and stdin as redirectable handles
- How pipes connect two processes at the OS level
- The REPL pattern — the foundation of shells, REPLs, and interpreters

---

## License

MIT
