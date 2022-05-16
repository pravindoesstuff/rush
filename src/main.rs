#[path = "internal_functions/cd.rs"]
mod cd;

const PROMPT: &str = "rush> ";

// Bring flush() into scope
use std::io::Write;

fn main() {
    loop {
        print!("{}", PROMPT);
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("{}", e);
            continue;
        }

        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("{}", e);
            continue;
        }

        let mut args = input.trim().split_whitespace();
        let command = match args.next() {
            Some(c) => c,
            None => continue,
        };

        match command {
            "cd" => {
                let dir = args.next().unwrap_or("~");
                if args.count() > 0 {
                    eprintln!("cd: too many arguments");
                } else {
                    cd::change_directory(dir);
                }
            }
            "exit" => {
                std::process::exit(0);
            }
            command => {
                let mut child = match std::process::Command::new(command).args(args).spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                if let Err(e) = child.wait() {
                    eprintln!("{}", e);
                    continue;
                }
            }
        }
    }
}
