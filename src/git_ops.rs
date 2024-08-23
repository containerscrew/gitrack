use colored::*;
use git2::{DiffOptions, Repository, StatusOptions};
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_excluded_dir(entry: &DirEntry, exclude_dirs: &[String]) -> bool {
    exclude_dirs.iter().any(|dir| entry.path().starts_with(dir))
}

pub fn find_git_repos(start_path: &Path, exclude_dirs: &[String]) -> Vec<PathBuf> {
    let mut git_repos = Vec::new();

    for entry in WalkDir::new(start_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_excluded_dir(e, exclude_dirs))
    {
        let path = entry.path();
        if path.is_dir() && path.join(".git").exists() {
            git_repos.push(path.to_path_buf());
        }
    }

    git_repos
}

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
