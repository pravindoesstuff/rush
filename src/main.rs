#[path = "internal_functions/cd.rs"]
mod cd;

#[path = "internal_functions/redirect.rs"]
mod redirect;

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

        let mut result = Vec::new();
        let mut last = 0;
        for (index, matched) in input.match_indices(|c| c == ' ' || c == '\n' || c == '|' || c == '>') {
            if last != index {
                result.push(&input[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }
        if last < input.len() {
            result.push(&input[last..]);
        }

        let mut commands = result.iter_mut().filter(|c| **c != " " && **c != "\n").peekable();
        let mut previous_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            match *command {
                "exit" => return,

                "cd" => {
                    if let Some(dir) = commands.next() {
                        cd::change_directory(dir)
                    } else {
                        cd::change_directory("~");
                    }
                    previous_command = None;
                    break;
                }

                ">" => {
                    if let Some(destination) = commands.next() {
                        if let Err(e) = redirect::redirect(destination, &mut previous_command) {
                            eprintln!("{}", e);
                        } 
                    } else {
                        eprintln!("Missing redirection destination");
                    }
                    previous_command = None;
                }

                "|" => {}

                command => {
                    let mut args = Vec::new();
                    while let Some(command) = commands.peek() {
                        if **command == "|" || **command == ">" {
                            break;
                        }
                        args.push(commands.next().unwrap());
                    }

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
