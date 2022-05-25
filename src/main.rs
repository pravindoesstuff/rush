#[path = "internal_functions/cd.rs"]
mod cd;

#[path = "internal_functions/redirect.rs"]
mod redirect;

#[path = "internal_functions/symbols.rs"]
mod symbols;

#[path = "internal_functions/parser.rs"]
mod parser;

const PROMPT: &str = "rush> ";
const HISTORY_FILE: &str = ".rush_history";

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    if rl.load_history(HISTORY_FILE).is_err() && std::fs::File::create(HISTORY_FILE).is_err() {
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

        let ast = parser::parse(&input);
        let mut split_command = parser::expand(&ast);
        let mut commands = split_command.iter_mut().peekable();

        let mut previous_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            match command.as_str() {
                "exit" => {
                    if let Err(e) = rl.save_history(HISTORY_FILE) {
                        eprintln!("Could not save history: {}", e);
                    }
                    return;
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

                ";" => {
                    if let Some(mut command) = previous_command {
                        if let Err(e) = command.wait() {
                            eprintln!("{}", e);
                        }
                    }
                    previous_command = None;
                }

                "&&" => {
                    if let Some(mut command) = previous_command {
                        match command.wait() {
                            Ok(status) => {
                                if !status.success() {
                                    previous_command = None;
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                    }
                    previous_command = None;
                }

                command => {
                    let mut args = Vec::new();
                    while let Some(command) = commands.peek() {
                        if symbols::is_protected(command) {
                            break;
                        }
                        args.push(commands.next().unwrap());
                    }

                    let stdin = match previous_command {
                        Some(child) => std::process::Stdio::from(child.stdout.unwrap()),
                        None => std::process::Stdio::inherit(),
                    };

                    let stdout = if commands.peek().is_some()
                        && !symbols::io_seperator(commands.peek().unwrap())
                    {
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
