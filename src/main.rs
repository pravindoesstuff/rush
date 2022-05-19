#[path = "internal_functions/cd.rs"]
mod cd;

#[path = "internal_functions/redirect.rs"]
mod redirect;

#[path = "internal_functions/interpreter.rs"]
mod interpreter;

const PROMPT: &str = "rush> ";
const HISTORY_FILE: &str = ".rush_history";

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    if rl.load_history(HISTORY_FILE).is_err() && std::fs::File::create(HISTORY_FILE).is_err()  {
        eprintln!("Could not read history file: {}", HISTORY_FILE);
    }

    loop {
        let input = match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(&line);
                line
            }

            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }

            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        let mut tokens = interpreter::parseline(&input);
        let mut commands = tokens.iter_mut().peekable();

        let mut previous_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            match *command {
                "exit" => {
                    if let Err(e) = rl.save_history(HISTORY_FILE) {
                        eprintln!("Could not save history: {}", e);
                    }
                    return
                }

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
