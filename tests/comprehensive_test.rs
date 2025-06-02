use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    fn run_drako(args: &[&str]) -> (bool, String, String) {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .output()
            .expect("Failed to execute drako");

        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        (success, stdout, stderr)
    }

    fn command_exists(cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[test]
    fn test_full_project_workflow() {
        let temp_dir = tempdir().unwrap();
        let project = temp_dir.path().join("full-project");

        let (success, _, stderr) = run_drako(&[
            project.to_str().unwrap(),
            "--git",
            "--npm",
            "--readme",
            "--license",
            "--docker",
            "--verbose",
            "-755",
        ]);

        assert!(success, "Full workflow should succeed: {}", stderr);
        assert!(project.exists());

        // Check permissions
        let mode = fs::metadata(&project).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o755);

        // Check files created
        assert!(project.join("README.md").exists());
        assert!(project.join("LICENSE").exists());
        assert!(project.join("Dockerfile").exists());
        assert!(project.join(".git").exists());

        if command_exists("npm") {
            assert!(project.join("package.json").exists());
        }
    }

    #[test]
    fn test_multiple_directories_same_config() {
        let temp_dir = tempdir().unwrap();
        let dirs: Vec<_> = (1..=3)
            .map(|i| temp_dir.path().join(format!("project{}", i)))
            .collect();

        let mut args: Vec<&str> = dirs.iter().map(|d| d.to_str().unwrap()).collect();
        args.extend(&["--readme", "--license", "-700"]);

        let (success, _, stderr) = run_drako(&args);
        assert!(success, "Multiple directories should succeed: {}", stderr);

        for dir in &dirs {
            assert!(dir.exists());
            assert!(dir.join("README.md").exists());
            assert!(dir.join("LICENSE").exists());

            let mode = fs::metadata(dir).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o700);
        }
    }

    #[test]
    fn test_nested_directory_creation() {
        let temp_dir = tempdir().unwrap();
        let nested = temp_dir.path().join("level1/level2/level3/project");

        let (success, _, stderr) =
            run_drako(&[nested.to_str().unwrap(), "--git", "--readme", "-755"]);

        assert!(success, "Nested creation should succeed: {}", stderr);
        assert!(nested.exists());
        assert!(nested.join("README.md").exists());
        assert!(nested.join(".git").exists());

        let mode = fs::metadata(&nested).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o755);
    }

    #[test]
    fn test_rust_project_initialization() {
        if !command_exists("cargo") {
            return;
        }

        let temp_dir = tempdir().unwrap();
        let rust_project = temp_dir.path().join("rust-project");

        let (success, _, stderr) = run_drako(&[
            rust_project.to_str().unwrap(),
            "--cargo",
            "--readme",
            "--license",
            "--git",
        ]);

        assert!(success, "Rust project should succeed: {}", stderr);
        assert!(rust_project.join("Cargo.toml").exists());
        assert!(rust_project.join("src/main.rs").exists());
        assert!(rust_project.join("README.md").exists());
        assert!(rust_project.join("LICENSE").exists());
        assert!(rust_project.join(".git").exists());
    }

    #[test]
    fn test_go_project_initialization() {
        if !command_exists("go") {
            return;
        }

        let temp_dir = tempdir().unwrap();
        let go_project = temp_dir.path().join("go-project");

        let (success, _, stderr) =
            run_drako(&[go_project.to_str().unwrap(), "--go", "--readme", "--docker"]);

        assert!(success, "Go project should succeed: {}", stderr);
        assert!(go_project.join("go.mod").exists());
        assert!(go_project.join("README.md").exists());
        assert!(go_project.join("Dockerfile").exists());

        let mod_content = fs::read_to_string(go_project.join("go.mod")).unwrap();
        assert!(mod_content.contains("module"));
        assert!(mod_content.contains("go-project"));
    }

    #[test]
    fn test_permission_variations() {
        let temp_dir = tempdir().unwrap();

        // Test different permission formats
        let test_cases = [("000", 0o000), ("7", 0o007), ("755", 0o755), ("777", 0o777)];

        for (perm_str, expected) in test_cases {
            let dir = temp_dir.path().join(format!("perm_{}", perm_str));
            let (success, _, _) = run_drako(&[dir.to_str().unwrap(), &format!("-{}", perm_str)]);

            assert!(success, "Permission {} should work", perm_str);
            let mode = fs::metadata(&dir).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, expected, "Permission {} mismatch", perm_str);
        }
    }

    #[test]
    fn test_file_content_validation() {
        let temp_dir = tempdir().unwrap();
        let project = temp_dir.path().join("content-test");

        let (success, _, _) = run_drako(&[
            project.to_str().unwrap(),
            "--readme",
            "--license",
            "--docker",
        ]);

        assert!(success);

        // Validate README
        let readme = fs::read_to_string(project.join("README.md")).unwrap();
        assert!(readme.contains("# Project Title"));
        assert!(readme.contains("## Description"));
        assert!(readme.contains("## Getting Started"));

        // Validate LICENSE
        let license = fs::read_to_string(project.join("LICENSE")).unwrap();
        assert!(license.contains("MIT License"));
        assert!(license.contains("Permission is hereby granted"));

        // Validate Dockerfile
        let dockerfile = fs::read_to_string(project.join("Dockerfile")).unwrap();
        assert!(dockerfile.contains("FROM"));
        assert!(dockerfile.contains("ARG STACK=node"));
        assert!(dockerfile.contains("EXPOSE 3000"));
    }

    #[test]
    fn test_error_handling() {
        let temp_dir = tempdir().unwrap();

        // Test invalid permissions - should still create directory
        let invalid_dir = temp_dir.path().join("invalid-perm");
        let (success, _, _) = run_drako(&[
            invalid_dir.to_str().unwrap(),
            "-999", // Invalid octal
            "--readme",
        ]);

        assert!(success);
        assert!(invalid_dir.exists());
        assert!(invalid_dir.join("README.md").exists());

        // Test unknown flags - should still work
        let unknown_dir = temp_dir.path().join("unknown-flag");
        let (success, _, stderr) =
            run_drako(&[unknown_dir.to_str().unwrap(), "--unknown-flag", "--readme"]);

        assert!(success);
        assert!(unknown_dir.exists());
        assert!(unknown_dir.join("README.md").exists());
        assert!(stderr.contains("Unknown flag"));
    }

    #[test]
    fn test_existing_directory_behavior() {
        let temp_dir = tempdir().unwrap();
        let existing = temp_dir.path().join("existing");

        fs::create_dir(&existing).unwrap();

        let (success, _, output) =
            run_drako(&[existing.to_str().unwrap(), "--readme", "--verbose"]);

        assert!(success);
        assert!(output.contains("already exists"));
        // Should not create files in existing directory
        assert!(!existing.join("README.md").exists());
    }

    #[test]
    fn test_help_and_version() {
        // Test help
        let (success, stdout, stderr) = run_drako(&["--help"]);
        assert!(success);
        let output = format!("{}{}", stdout, stderr);
        assert!(output.contains("Usage:"));
        assert!(output.contains("--git"));
        assert!(output.contains("--readme"));

        // Test version
        let (success, stdout, stderr) = run_drako(&["--version"]);
        assert!(success);
        let output = format!("{}{}", stdout, stderr);
        assert!(output.contains("drako"));
    }

    #[test]
    fn test_no_arguments() {
        let (success, stdout, stderr) = run_drako(&[]);
        assert!(!success);
        let output = format!("{}{}", stdout, stderr);
        assert!(output.contains("Usage:"));
    }

    #[test]
    fn test_special_characters_in_paths() {
        let temp_dir = tempdir().unwrap();
        let special = temp_dir.path().join("test-dir_with.special@chars");

        let (success, _, _) = run_drako(&[special.to_str().unwrap(), "--readme", "-777"]);

        assert!(success);
        assert!(special.exists());
        assert!(special.join("README.md").exists());

        let mode = fs::metadata(&special).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o777);
    }

    #[test]
    fn test_verbose_output() {
        let temp_dir = tempdir().unwrap();
        let project = temp_dir.path().join("verbose-test");

        let (success, stdout, stderr) = run_drako(&[
            project.to_str().unwrap(),
            "--readme",
            "--git",
            "--verbose",
            "-755",
        ]);

        assert!(success);
        let output = format!("{}{}", stdout, stderr);
        assert!(output.contains("Created") || output.contains("755"));
    }
}
