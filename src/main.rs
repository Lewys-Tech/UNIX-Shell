use std::io::{self, BufRead, Write};
use std::process::Command;
fn main( ){
    let stdin = io::stdin();

    loop {
        print!("rush> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let line = line.trim();


        if line == "exit" {
            break;
        }

        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        if cmd == "cd" {
            let dir = args.get(0).unwrap_or(&"");
            std::env::set_current_dir(dir).unwrap();
            continue
        }

        Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    }
}