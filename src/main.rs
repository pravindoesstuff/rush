const PROMPT: &str = "rush> ";

use std::io::Write;

fn main() {
    loop {
        print!("{}", PROMPT);
        if let Err(e)  = std::io::stdout().flush() {
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

        let mut child = match std::process::Command::new(command)
            .args(args)
            .spawn() {
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
