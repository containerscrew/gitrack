use crate::git_ops::{check_untracked_files, find_git_repos, show_diff};
use clap::Parser;
use cli::Args;
use colored::*;
use git_ops::pull_changes;
use std::path::Path;

mod cli;
mod git_ops;
mod test;
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
    // Initialize the CLI arguments
    let args = Args::parse();

    println_orange!("----->  Scanning {}  <-----", args.path);

    // Find .git repos in the specified path
    let start_path = Path::new(&args.path);
    let exclude_dirs = args.exclude.as_deref().unwrap_or(&[]);
    if !exclude_dirs.is_empty() {
        println_orange!("----->  Excluding directories: {:?}", exclude_dirs);
    }

    // Start scanning
    let git_repos = find_git_repos(start_path, exclude_dirs, args.workers as usize);

    // Process each repository
    for repo in git_repos {
        // Git pull to update the repository
        if args.pull {
            println_orange!("Pulling latest changes in: {}", repo.display());
            match pull_changes(&repo) {
                Ok(result) => println_orange!(
                    "Successfully pulled changes in: {}. {}",
                    repo.display(),
                    result
                ),
                Err(e) => println_light_orange!("{}: {}", "Error pulling changes".red(), e),
            }
        } else {
            process_repo(&repo, &args);
        }
    }
}

/// Process each repository based on command-line arguments
fn process_repo(repo_path: &Path, args: &Args) {
    if args.check_untracked {
        match check_untracked_files(repo_path) {
            Ok(untracked_files) => handle_untracked_files(repo_path, &untracked_files, args),
            Err(e) => {
                if args.verbose {
                    eprintln!("{}: {}", "Error checking repository".red(), e);
                }
            }
        }
    } else {
        println_orange!("Git repository found: {}", repo_path.display());
    }
}

/// Handle the display of untracked files
fn handle_untracked_files(repo_path: &Path, untracked_files: &[String], args: &Args) {
    if !untracked_files.is_empty() {
        println_orange!("Untracked files in: {}", repo_path.display());
        for file in untracked_files {
            if args.verbose {
                println_light_orange!("  - {}", file);
            }
            if args.diff {
                match show_diff(repo_path, file) {
                    Ok(diff) => println!("{}", diff),
                    Err(e) => eprintln!("{}: {}", "Error showing diff".red(), e),
                }
            }
        }
    }
}
