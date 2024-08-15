
pub fn default_home_path() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}