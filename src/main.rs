use clap::Parser;
use cli::Args;
use colored::*;
use std::path::{Path, PathBuf};
use crate::git_ops::{check_untracked_files, find_git_repos};

mod cli;
mod git_ops;
mod utils;

// Macro print orange
macro_rules! println_orange {
    ($($arg:tt)*) => {{
        println!("{}", format!($($arg)*).truecolor(255, 165, 0));
    }};
}

// Macro print light orange
macro_rules! println_light_orange {
    ($($arg:tt)*) => {{
        println!("{}", format!($($arg)*).truecolor(255, 200, 100));
    }};
}

fn main() {
    // Init the CLI using clap
    let args = Args::parse();

    println!("{}", "----->  Starting gitrack  <-----".green());

    // Find .git repos in the given path
    let start_path = Path::new(&args.path);
    let git_repos = find_git_repos(start_path);

    let mut results = Vec::new();

    // For every repo, check for untracked files
    for repo_path in &git_repos {
        match check_untracked_files(&repo_path) {
            Ok(untracked_files) => {
                if !untracked_files.is_empty() {
                    results.push((repo_path.clone(), untracked_files));
                }
            }
            Err(e) => eprintln!("{}: {}", "Error checking repository".red(), e),
        }
    }

    // Print the results, if any
    match results.is_empty() {
        true => println_orange!("-----> No git repos found in {}", start_path.display()),
        false => print_results(results, args.summary),
    }
}

fn print_results(results: Vec<(PathBuf, Vec<String>)>, summary: bool) {
    for (repo_path, untracked_files) in results {
        println_orange!("Untracked files in: {}", repo_path.display());
        if !summary {
            for file in untracked_files {
                println_light_orange!("  - {}", file);
            }
        }
    }
}