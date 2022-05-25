#[path = "symbols.rs"]
mod symbols;

enum State {
    SingleQuoted,
    DoubleQuoted,
    Unquoted,
    DoubleOperator,
}

#[derive(Debug)]
pub enum Argument {
    /// Shell expansions may not be performed
    Quoted(String),
    /// Shell expansions may be performed
    Unquoted(String),
    /// Operator
    Operator(String),
}

pub fn expand(arguments: &Vec<Argument>) -> Vec<&str> {
    let mut expanded = Vec::new();
    for a in arguments {
        expanded.push(match a {
            Argument::Quoted(a) | Argument::Unquoted(a) | Argument::Operator(a) => a.as_ref(),
        });
    }
    expanded
}

pub fn parse(command: &str) -> Vec<Argument> {
    let mut chunks = Vec::new();
    let mut chunk = String::new();

    let mut state = State::Unquoted;
    for c in command.chars() {
        match state {
            State::SingleQuoted => match c {
                '\'' => {
                    state = State::Unquoted;
                    chunks.push(Argument::Quoted(std::mem::take(&mut chunk)));
                }
                c => chunk.push(c),
            },
            State::DoubleQuoted => match c {
                '"' => {
                    state = State::Unquoted;
                    chunks.push(Argument::Quoted(std::mem::take(&mut chunk)));
                }
                c => chunk.push(c),
            },
            State::DoubleOperator => {
                if chunk == String::from(c) {
                    chunk.push(c);
                }
                chunks.push(Argument::Quoted(std::mem::take(&mut chunk)));
                state = State::Unquoted;
            },

            State::Unquoted => {
                if c.is_whitespace() {
                    state = State::Unquoted;
                    if !chunk.is_empty() {
                        chunks.push(Argument::Unquoted(std::mem::take(&mut chunk)));
                    }
                } else if c == '\'' {
                    state = State::SingleQuoted;
                    if !chunk.is_empty() {
                        chunks.push(Argument::Unquoted(std::mem::take(&mut chunk)));
                    }
                } else if c == '"' {
                    state = State::DoubleQuoted;
                    if !chunk.is_empty() {
                        chunks.push(Argument::Unquoted(std::mem::take(&mut chunk)));
                    }
                } else if symbols::is_protected(&String::from(c)) {
                    if !chunk.is_empty() {
                        chunks.push(Argument::Unquoted(std::mem::take(&mut chunk)));
                    }
                    if c != '&' && c != '|' {
                        chunks.push(Argument::Operator(String::from(c)));
                    } else {
                        chunk.push(c);
                        state = State::DoubleOperator;
                    }
                } else {
                    chunk.push(c);
                }
            }
        }
    }
    if !chunk.is_empty() {
        chunks.push(Argument::Unquoted(chunk));
    }
    chunks
}
