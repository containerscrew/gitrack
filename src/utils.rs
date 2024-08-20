use colored::Colorize;

pub fn default_home_dir() -> String {
    match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path.display().to_string(),
        _ => {
            println!("{}", "Could not find home directory, using root '/'".red());
            "/".to_string()
        }
    }
}