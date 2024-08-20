use crate::git_ops::{check_untracked_files, find_git_repos};
use clap::Parser;
use cli::Args;
use colored::*;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

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

    println!("{}", "----->  Starting gitrack  <-----".green());

    // Find .git repos in the specified path
    let start_path = Path::new(&args.path);
    let git_repos = find_git_repos(start_path);

    // Create a channel to send the results back to the main thread
    let (tx, rx) = mpsc::channel();

    // Spawn a thread for each repo to check for untracked files
    for repo_path in git_repos {
        let tx = tx.clone();
        thread::spawn(move || {
            match check_untracked_files(&repo_path) {
                Ok(untracked_files) => {
                    if !untracked_files.is_empty() {
                        // Send the results back to the main thread
                        tx.send((repo_path.clone(), untracked_files)).unwrap();
                    }
                }
                Err(e) => eprintln!("{}: {}", "Error checking repository".red(), e),
            }
        });
    }

    // Close the sending side of the channel
    drop(tx);

    // Print the results as they arrive
    let mut has_results = false;
    while let Ok((repo_path, untracked_files)) = rx.recv() {
        has_results = true;
        println_orange!("Untracked files in: {}", repo_path.display());
        if !args.summary {
            for file in untracked_files {
                println_light_orange!("  - {}", file);
            }
        }
    }

    // Print a message if no results were found
    if !has_results {
        println_orange!(
            "-----> There are no changes to git in {}",
            start_path.display()
        );
    }
}
