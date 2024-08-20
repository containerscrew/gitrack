use git2::{Repository, StatusOptions};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_git_repos(start_path: &Path) -> Vec<PathBuf> {
    let mut git_repos = Vec::new();
    for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
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
