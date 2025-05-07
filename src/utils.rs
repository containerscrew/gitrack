use std::io;

use colored::Colorize;
use which::which;

pub fn default_home_dir() -> String {
    match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path.display().to_string(),
        _ => {
            println!("{}", "Could not find home directory, using root '/'".red());
            "/".to_string()
        }
    }
}

pub fn find_binary_path(binary_name: &str) -> Result<std::path::PathBuf, io::Error> {
    which(binary_name).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("'{}' not found: {}", binary_name, e),
        )
    })
}
