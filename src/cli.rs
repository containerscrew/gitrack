use colored::*;

use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(
    about = "untracked",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Inspect your untracked local Git files",
    arg_required_else_help = false
)]

pub struct Args {
    #[arg(
        short = 'p',
        long = "path",
        help = "Folder path you want to scan for git untracked files",
        default_value_t = default_home_dir(),
        required = false,
    )]
    pub path: String,

    #[arg(
        short = 's',
        long = "summary",
        help = "Show only repositories without listing untracked files",
        required = false,
    )]
    pub summary: bool,
}


fn default_home_dir() -> String {
    match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path.display().to_string(),
        _ => {
            println!("{}", "Could not find home directory, using root '/'".red());
            "/".to_string()
        }
    }
}