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

        let mut commands = input.trim().split('|').peekable();

        let mut previous_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            let mut args = command.trim().split_whitespace();
            let command = match args.next() {
                Some(c) => c,
                None => continue,
            };

            match command {
                "exit" => return,

                "cd" => {
                    let dir = args.next().unwrap_or("~");
                    if args.count() > 0 {
                        eprintln!("cd: too many arguments");
                        break;
                    } else {
                        cd::change_directory(dir);
                    }
                    previous_command = None;
                }

                command => {
                    let stdin = match previous_command {
                        Some(child) => std::process::Stdio::from(child.stdout.unwrap()),
                        None => std::process::Stdio::inherit(),
                    };

                    let stdout = if commands.peek().is_some() {
                        std::process::Stdio::piped()
                    } else {
                        std::process::Stdio::inherit()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            if let Err(e) = final_command.wait() {
                eprintln!("{}", e);
                continue;
            }
        }
    }
}
