#[cfg(test)]
mod flag_parsing_tests {
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::io::Write;
    use tempfile::tempdir;

    // Helper function to run the makedir binary with specific arguments
    fn run_makedir(args: &[&str]) -> (bool, String, String) {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .output()
            .expect("Failed to execute makedir");
        
        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        (success, stdout, stderr)
    }

    #[test]
    fn test_verbose_flag_short() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_verbose_short");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, stdout, stderr) = run_makedir(&[test_dir_str, "-v"]);
        
        assert!(success, "Command should succeed");
        assert!(stdout.contains("Creating directory"), "Verbose output should be shown");
        assert!(!stderr.contains("Unknown flag"), "Should not show unknown flag error");
        assert!(test_dir.exists(), "Directory should be created");
    }

    #[test]
    fn test_verbose_flag_long() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_verbose_long");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, stdout, stderr) = run_makedir(&[test_dir_str, "--verbose"]);
        
        assert!(success, "Command should succeed");
        assert!(stdout.contains("Creating directory"), "Verbose output should be shown");
        assert!(!stderr.contains("Unknown flag"), "Should not show unknown flag error");
        assert!(test_dir.exists(), "Directory should be created");
    }

    #[test]
    fn test_permission_flag() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_permissions");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, _, stderr) = run_makedir(&[test_dir_str, "-700"]);
        
        assert!(success, "Command should succeed");
        assert!(!stderr.contains("Unknown flag"), "Should not show unknown flag error");
        assert!(test_dir.exists(), "Directory should be created");
        
        // On Unix-like systems, check the actual permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&test_dir).unwrap();
            let mode = metadata.permissions().mode() & 0o777;
            assert_eq!(mode, 0o700, "Directory should have 700 permissions");
        }
    }

    #[test]
    fn test_multiple_flags() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_multiple_flags");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, stdout, stderr) = run_makedir(&[test_dir_str, "-v", "-r", "-l", "-700"]);
        
        assert!(success, "Command should succeed");
        assert!(stdout.contains("Creating directory"), "Verbose output should be shown");
        assert!(!stderr.contains("Unknown flag"), "Should not show unknown flag error");
        assert!(test_dir.exists(), "Directory should be created");
        assert!(test_dir.join("README.md").exists(), "README.md should be created");
        assert!(test_dir.join("LICENSE").exists(), "LICENSE should be created");
        
        // Check permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&test_dir).unwrap();
            let mode = metadata.permissions().mode() & 0o777;
            assert_eq!(mode, 0o700, "Directory should have 700 permissions");
        }
    }

    #[test]
    fn test_invalid_permission_format() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_invalid_perm");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, _, stderr) = run_makedir(&[test_dir_str, "-9999"]);
        
        assert!(success, "Command should still succeed even with invalid permission");
        assert!(stderr.contains("Invalid permission format"), "Should show invalid permission error");
        assert!(test_dir.exists(), "Directory should still be created");
    }

    #[test]
    fn test_unknown_flag() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_unknown_flag");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, _, stderr) = run_makedir(&[test_dir_str, "--nonexistent-flag"]);
        
        assert!(success, "Command should still succeed");
        assert!(stderr.contains("Unknown flag"), "Should show unknown flag error");
        assert!(test_dir.exists(), "Directory should still be created");
    }

    #[test]
    fn test_edge_case_dash_only() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_dash_only");
        let test_dir_str = test_dir.to_str().unwrap();
        
        let (success, _, _) = run_makedir(&[test_dir_str, "-"]);
        
        assert!(success, "Command should succeed");
        assert!(test_dir.exists(), "Directory should be created");
        // The "-" should be treated as a flag but not recognized, so it should show an unknown flag error
    }

    #[test]
    fn test_edge_case_directory_with_dash() {
        let temp_dir = tempdir().unwrap();
        let dash_dir = "-dash-directory";
        let test_dir = temp_dir.path().join(dash_dir);
        let test_dir_str = test_dir.to_str().unwrap();
        
        // Need to use -- to indicate end of options
        let (success, _, _) = run_makedir(&["--", test_dir_str]);
        
        assert!(success, "Command should succeed");
        assert!(test_dir.exists(), "Directory with dash should be created");
    }

    #[test]
    fn test_multiple_directories() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");
        let dir3 = temp_dir.path().join("dir3");
        
        let (success, _, _) = run_makedir(&[
            dir1.to_str().unwrap(),
            dir2.to_str().unwrap(),
            dir3.to_str().unwrap(),
            "-v"
        ]);
        
        assert!(success, "Command should succeed");
        assert!(dir1.exists(), "First directory should be created");
        assert!(dir2.exists(), "Second directory should be created");
        assert!(dir3.exists(), "Third directory should be created");
    }

    #[test]
    fn test_existing_directory() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("existing_dir");
        fs::create_dir(&test_dir).unwrap();
        
        let (success, stdout, _) = run_makedir(&[test_dir.to_str().unwrap(), "-v"]);
        
        assert!(success, "Command should succeed");
        assert!(stdout.contains("Directory already exists"), "Should indicate directory exists");
    }

    #[test]
    fn test_no_arguments() {
        let (success, _, stderr) = run_makedir(&[]);
        
        assert!(!success, "Command should fail with no arguments");
        assert!(stderr.contains("Usage:"), "Should show usage information");
    }
}