use colored::*;
use git2::{DiffOptions, Repository, StatusOptions};
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use walkdir::WalkDir;

use crate::utils::find_binary_path;

// Check if a directory should be excluded
fn is_excluded_dir(entry: &walkdir::DirEntry, exclude_dirs: &[String]) -> bool {
    let path = entry.path(); // Get the full path of the directory
    exclude_dirs
        .iter()
        .any(|exclude| path.to_string_lossy().contains(exclude))
}

// Send the path to the channel if it is a Git repository
fn check_and_send_repo(path: PathBuf, tx: Sender<PathBuf>) {
    if path.join(".git").exists() {
        tx.send(path).unwrap();
    }
}

// Find Git repositories starting from a given directory using a ThreadPool
pub fn find_git_repos(
    start_path: &Path,
    exclude_dirs: &[String],
    num_threads: usize,
) -> Vec<PathBuf> {
    let pool = ThreadPool::new(num_threads); // Create a thread pool with the specified number of threads
    let (tx, rx) = mpsc::channel(); // Create a channel to send results from threads to the main thread

    // Iterate over all entries in the starting path
    for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();
        if path.is_dir() && !is_excluded_dir(&entry, exclude_dirs) {
            let tx = tx.clone(); // Clone the sender to be used in the thread
            pool.execute(move || {
                check_and_send_repo(path, tx); // Check if the directory is a Git repository and send the path if it is
            });
        }
    }

    drop(tx); // Close the sender side of the channel to indicate no more sends

    // Collect all the paths from the receiver into a vector
    rx.into_iter().collect()
}

// Check for untracked files in a Git repository
pub fn check_untracked_files(repo_path: &Path) -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_options))?;

    let mut untracked_files = Vec::new();
    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_index_new() || status.is_wt_new() || status.is_wt_modified() {
            let file_path = entry.path().unwrap_or("unknown file").to_string();
            untracked_files.push(file_path);
        }
    }
    Ok(untracked_files)
}

// Pull changes in a Git repository
// TODO: use git2 library instead of command process, like the rest of the code
pub fn pull_changes(repo_path: &Path) -> Result<String, io::Error> {
    let git_path = find_binary_path("git").unwrap_or_else(|_| "/usr/bin/git".into());

    // Ejecutar git pull en el directorio correcto
    let output = Command::new(git_path)
        .arg("pull")
        .arg("origin")
        .arg("HEAD")
        .current_dir(repo_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(ErrorKind::Other, error_msg.to_string()))
    }
}

// Show the diff for a specific file in a Git repository
pub fn show_diff(repo_path: &Path, file: &str) -> io::Result<String> {
    let repo = Repository::open(repo_path).expect("Error opening repository");
    let mut diff_options = DiffOptions::new();
    diff_options.pathspec(file);

    let diff = repo
        .diff_index_to_workdir(None, Some(&mut diff_options))
        .expect("Error diffing");
    let mut diff_output = Vec::new();

    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        let content = String::from_utf8_lossy(line.content()).to_string();
        let colored_line = match line.origin() {
            '-' => content.red().to_string(),   // Deleted lines in red
            '+' => content.green().to_string(), // Added lines in green
            _ => content,                       // Unchanged lines
        };
        diff_output.push(colored_line);
        true
    })
    .expect("Error printing diff");

    if !diff_output.is_empty() {
        Ok(diff_output.join(""))
    } else {
        Err(io::Error::new(
            ErrorKind::NotFound,
            format!("No differences found for {}", file),
        ))
    }
}
