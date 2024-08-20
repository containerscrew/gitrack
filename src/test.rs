#[cfg(test)]
mod tests {
    use crate::utils::default_home_dir;
    use std::env;
    use std::path::PathBuf;

    // Helper function to temporarily modify the $HOME environment variable
    fn with_temp_home<R, F: FnOnce() -> R>(temp_home: &str, test: F) -> R {
        // Save the original $HOME value
        let original_home = env::var("HOME").ok();

        // Set the $HOME variable temporarily
        env::set_var("HOME", temp_home);

        // Execute the test
        let result = test();

        // Restore the original $HOME value
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        result
    }

    #[test]
    fn returns_home_directory_when_available() {
        with_temp_home("/home/testuser", || {
            let home_path = PathBuf::from("/home/testuser");
            assert_eq!(default_home_dir(), home_path.display().to_string());
        });
    }

    #[test]
    fn returns_root_when_home_directory_is_not_available() {
        with_temp_home("", || {
            assert_eq!(default_home_dir(), "/".to_string());
        });
    }
}
