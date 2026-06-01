use std::io::{self, BufRead, Write};
use std::process::Command;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("rush> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        if cmd == "exit" {
            std::process::exit(0);
        }

        if cmd == "help" {
            println!("Built-in commands: cd, help, exit");
            continue;
        }

        if cmd == "cd" {
            let dir = args.get(0).unwrap_or(&"");
            std::env::set_current_dir(dir).unwrap();
            continue;
        }
        if let Some(pos) = args.iter().position(|&a| a == "|"){
            let left_args: Vec<&str>  =  args[..pos].to_vec();
            let right_cmd = args[pos + 1];
            let right_args: Vec<&str> = args
        }




        if let Some(pos) = args.iter().position(|&a| a == ">"){
            let filename = args[pos + 1];
            let file = std::fs::File::create(filename).unwrap();
            let args: Vec<&str> = args[..pos].to_vec();
        

            Command::new(cmd)
                .args(args)
                .stdout(file)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            continue;
        }
            Command::new(cmd)
            .args(args)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }    
}