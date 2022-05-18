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

        let text = &input;

        let mut result = Vec::new();
        let mut last = 0;
        for (index, matched) in text.match_indices(|c| c == ' ' || c == '|' || c == '>') {
            if last != index {
                result.push(&text[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }
        if last < text.len() {
            result.push(&text[last..]);
        }

        let mut commands = result.iter_mut().filter(|c| c != &&" ").peekable();
        let mut previous_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            match command.trim_end() {
                "exit" => return,

                "cd" => {
                    if let Some(dir) = commands.next() {
                        cd::change_directory(dir.trim_end());
                    } else {
                        cd::change_directory("~");
                    }
                    previous_command = None;
                    break;
                }

                command => {
                    let mut args = Vec::new();
                    for command in commands.by_ref() {
                        if command == &"|" || command == &">" {
                            break;
                        }
                        let command = command.trim_end();
                        args.push(command);
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
