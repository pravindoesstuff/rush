use std::str::FromStr;

pub fn change_directory(directory: &str) {
    let path = if directory == "~" {
        match home::home_dir() {
            Some(p) => p,
            None => {
                eprintln!("cd: Unable to determine the home directory");
                return;
            }
        }
    } else {
        match std::path::PathBuf::from_str(directory) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("cd: {}", e);
                return;
            }
        }
    };
    if let Err(e) = std::env::set_current_dir(path) {
        eprintln!("cd: {}", e);
    }
}
