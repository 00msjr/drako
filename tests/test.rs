use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[cfg(test)]
mod advanced_integration_tests {
    use super::*;

    // Helper function to run drako with arguments and capture all output
    fn run_drako_with_output(args: &[&str]) -> (bool, String, String) {
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

    // Helper function to check if a command exists on the system
    fn command_exists(cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    // Helper function to count lines in a file
    fn count_file_lines(path: &Path) -> usize {
        fs::read_to_string(path).unwrap_or_default().lines().count()
    }

    #[test]
    fn test_complex_workflow_full_stack_project() {
        let temp_dir = tempdir().unwrap();
        let project_dir = temp_dir.path().join("my-full-stack-app");

        // Create a project with multiple initializations
        let (success, stdout, stderr) = run_drako_with_output(&[
            project_dir.to_str().unwrap(),
            "--git",
            "--npm",
            "--readme",
            "--license",
            "--docker",
            "--verbose",
            "-755",
        ]);

        assert!(success, "Command should succeed. stderr: {}", stderr);
        assert!(project_dir.exists(), "Project directory should exist");

        // Verify permissions
        let metadata = fs::metadata(&project_dir).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o755, "Directory should have 755 permissions");

        // Verify git initialization
        assert!(
            project_dir.join(".git").exists(),
            "Git repository should be initialized"
        );

        // Verify package.json creation (if npm is available)
        if command_exists("npm") {
            assert!(
                project_dir.join("package.json").exists(),
                "package.json should be created"
            );
        }

        // Verify template files
        assert!(
            project_dir.join("README.md").exists(),
            "README.md should exist"
        );
        assert!(project_dir.join("LICENSE").exists(), "LICENSE should exist");
        assert!(
            project_dir.join("Dockerfile").exists(),
            "Dockerfile should exist"
        );

        // Verify README content
        let readme_content = fs::read_to_string(project_dir.join("README.md")).unwrap();
        assert!(
            readme_content.contains("# Project Title"),
            "README should have correct title"
        );
        assert!(
            readme_content.contains("## Description"),
            "README should have description section"
        );

        // Verify LICENSE content
        let license_content = fs::read_to_string(project_dir.join("LICENSE")).unwrap();
        assert!(
            license_content.contains("MIT License"),
            "LICENSE should be MIT license"
        );
        assert!(
            license_content.contains("Permission is hereby granted"),
            "LICENSE should have permission text"
        );

        // Verify Dockerfile content
        let dockerfile_content = fs::read_to_string(project_dir.join("Dockerfile")).unwrap();
        assert!(
            dockerfile_content.contains("FROM"),
            "Dockerfile should have FROM instruction"
        );
        assert!(
            dockerfile_content.contains("ARG STACK=node"),
            "Dockerfile should have STACK argument"
        );

        // Verify verbose output
        assert!(
            stdout.contains("Created directory") || stderr.contains("Created directory"),
            "Verbose output should show directory creation"
        );
    }

    #[test]
    fn test_multiple_directories_with_different_permissions() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("project1");
        let dir2 = temp_dir.path().join("project2");
        let dir3 = temp_dir.path().join("project3");

        // Test creating multiple directories with same permissions
        let (success, _, stderr) = run_drako_with_output(&[
            dir1.to_str().unwrap(),
            dir2.to_str().unwrap(),
            dir3.to_str().unwrap(),
            "-700",
            "--readme",
            "--verbose",
        ]);

        assert!(success, "Command should succeed. stderr: {}", stderr);

        // Verify all directories exist
        assert!(dir1.exists(), "Directory 1 should exist");
        assert!(dir2.exists(), "Directory 2 should exist");
        assert!(dir3.exists(), "Directory 3 should exist");

        // Verify permissions on all directories
        for dir in [&dir1, &dir2, &dir3] {
            let metadata = fs::metadata(dir).unwrap();
            let mode = metadata.permissions().mode() & 0o777;
            assert_eq!(mode, 0o700, "All directories should have 700 permissions");
        }

        // Verify README files in all directories
        assert!(
            dir1.join("README.md").exists(),
            "README should exist in dir1"
        );
        assert!(
            dir2.join("README.md").exists(),
            "README should exist in dir2"
        );
        assert!(
            dir3.join("README.md").exists(),
            "README should exist in dir3"
        );
    }

    #[test]
    fn test_deeply_nested_directory_creation() {
        let temp_dir = tempdir().unwrap();
        let deep_path = temp_dir
            .path()
            .join("level1")
            .join("level2")
            .join("level3")
            .join("level4")
            .join("final-project");

        let (success, _, stderr) =
            run_drako_with_output(&[deep_path.to_str().unwrap(), "--git", "--readme", "-755"]);

        assert!(success, "Command should succeed. stderr: {}", stderr);
        assert!(deep_path.exists(), "Deeply nested directory should exist");

        // Verify only the final directory has the specified permissions
        let metadata = fs::metadata(&deep_path).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o755, "Final directory should have 755 permissions");

        // Verify parent directories have default permissions (not 755)
        let parent = deep_path.parent().unwrap();
        let parent_metadata = fs::metadata(parent).unwrap();
        let parent_mode = parent_metadata.permissions().mode() & 0o777;
        assert_ne!(
            parent_mode, 0o755,
            "Parent directory should not have same permissions"
        );

        // Verify files were created in the correct location
        assert!(
            deep_path.join("README.md").exists(),
            "README should be in final directory"
        );
        assert!(
            deep_path.join(".git").exists(),
            "Git should be initialized in final directory"
        );
    }

    #[test]
    fn test_rust_cargo_project_initialization() {
        // Skip if cargo is not available
        if !command_exists("cargo") {
            println!("Skipping Rust test - cargo not available");
            return;
        }

        let temp_dir = tempdir().unwrap();
        let rust_project = temp_dir.path().join("my-rust-project");

        let (success, _, stderr) = run_drako_with_output(&[
            rust_project.to_str().unwrap(),
            "--cargo",
            "--readme",
            "--license",
            "--git",
            "--verbose",
        ]);

        assert!(
            success,
            "Rust project creation should succeed. stderr: {}",
            stderr
        );
        assert!(rust_project.exists(), "Rust project directory should exist");

        // Verify Cargo.toml was created
        assert!(
            rust_project.join("Cargo.toml").exists(),
            "Cargo.toml should be created"
        );

        // Verify src directory and main.rs were created by cargo init
        assert!(
            rust_project.join("src").exists(),
            "src directory should exist"
        );
        assert!(
            rust_project.join("src/main.rs").exists(),
            "main.rs should exist"
        );

        // Verify our template files
        assert!(
            rust_project.join("README.md").exists(),
            "README.md should exist"
        );
        assert!(
            rust_project.join("LICENSE").exists(),
            "LICENSE should exist"
        );
        assert!(
            rust_project.join(".git").exists(),
            "Git should be initialized"
        );

        // Verify Cargo.toml content
        let cargo_content = fs::read_to_string(rust_project.join("Cargo.toml")).unwrap();
        assert!(
            cargo_content.contains("[package]"),
            "Cargo.toml should have package section"
        );
        assert!(
            cargo_content.contains("name = "),
            "Cargo.toml should have project name"
        );
    }

    #[test]
    fn test_go_project_initialization() {
        // Skip if go is not available
        if !command_exists("go") {
            println!("Skipping Go test - go not available");
            return;
        }

        let temp_dir = tempdir().unwrap();
        let go_project = temp_dir.path().join("my-go-project");

        let (success, _, stderr) = run_drako_with_output(&[
            go_project.to_str().unwrap(),
            "--go",
            "--readme",
            "--docker",
            "--verbose",
        ]);

        assert!(
            success,
            "Go project creation should succeed. stderr: {}",
            stderr
        );
        assert!(go_project.exists(), "Go project directory should exist");

        // Verify go.mod was created
        assert!(
            go_project.join("go.mod").exists(),
            "go.mod should be created"
        );

        // Verify go.mod content
        let go_mod_content = fs::read_to_string(go_project.join("go.mod")).unwrap();
        assert!(
            go_mod_content.contains("module"),
            "go.mod should have module declaration"
        );
        assert!(
            go_mod_content.contains("my-go-project"),
            "go.mod should reference project name"
        );

        // Verify template files
        assert!(
            go_project.join("README.md").exists(),
            "README.md should exist"
        );
        assert!(
            go_project.join("Dockerfile").exists(),
            "Dockerfile should exist"
        );
    }

    #[test]
    fn test_edge_case_special_characters_in_paths() {
        let temp_dir = tempdir().unwrap();
        let special_dir = temp_dir.path().join("test-dir_with.special@chars");

        let (success, _, stderr) =
            run_drako_with_output(&[special_dir.to_str().unwrap(), "--readme", "-777"]);

        assert!(
            success,
            "Command should handle special characters. stderr: {}",
            stderr
        );
        assert!(
            special_dir.exists(),
            "Directory with special chars should exist"
        );

        let metadata = fs::metadata(&special_dir).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o777, "Directory should have 777 permissions");

        assert!(
            special_dir.join("README.md").exists(),
            "README should be created"
        );
    }

    #[test]
    fn test_permission_edge_cases() {
        let temp_dir = tempdir().unwrap();

        // Test minimum permissions (000)
        let no_perm_dir = temp_dir.path().join("no_permissions");
        let (success, _, _) = run_drako_with_output(&[no_perm_dir.to_str().unwrap(), "-000"]);
        assert!(success, "Should handle 000 permissions");
        let mode = fs::metadata(&no_perm_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o000);

        // Test maximum permissions (777)
        let full_perm_dir = temp_dir.path().join("full_permissions");
        let (success, _, _) = run_drako_with_output(&[full_perm_dir.to_str().unwrap(), "-777"]);
        assert!(success, "Should handle 777 permissions");
        let mode = fs::metadata(&full_perm_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o777);

        // Test single digit permissions
        let single_digit_dir = temp_dir.path().join("single_digit");
        let (success, _, _) = run_drako_with_output(&[single_digit_dir.to_str().unwrap(), "-7"]);
        assert!(success, "Should handle single digit permissions");
        let mode = fs::metadata(&single_digit_dir)
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o007);
    }

    #[test]
    fn test_concurrent_operations_simulation() {
        let temp_dir = tempdir().unwrap();
        let base_name = "concurrent_test";

        // Simulate multiple operations that might conflict
        let dirs: Vec<String> = (0..5)
            .map(|i| {
                temp_dir
                    .path()
                    .join(format!("{}_{}", base_name, i))
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        let mut dir_args: Vec<&str> = dirs.iter().map(|s| s.as_str()).collect();
        dir_args.extend_from_slice(&["--git", "--readme", "--license", "-755", "--verbose"]);

        let (success, _, stderr) = run_drako_with_output(&dir_args);

        assert!(
            success,
            "Concurrent-style operations should succeed. stderr: {}",
            stderr
        );

        // Verify all directories were created
        for (i, dir_path) in dirs.iter().enumerate() {
            let path = Path::new(dir_path);
            assert!(path.exists(), "Directory {} should exist", i);
            assert!(path.join("README.md").exists(), "README {} should exist", i);
            assert!(path.join("LICENSE").exists(), "LICENSE {} should exist", i);
            assert!(
                path.join(".git").exists(),
                "Git {} should be initialized",
                i
            );

            let mode = fs::metadata(path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o755, "Directory {} should have 755 permissions", i);
        }
    }

    #[test]
    fn test_help_and_version_outputs() {
        // Test help output
        let (success, stdout, stderr) = run_drako_with_output(&["--help"]);
        assert!(success, "Help command should succeed");
        let output = format!("{}{}", stdout, stderr);
        assert!(
            output.contains("Usage:"),
            "Help should contain usage information"
        );
        assert!(output.contains("--git"), "Help should list git option");
        assert!(
            output.contains("--readme"),
            "Help should list readme option"
        );
        assert!(
            output.contains("--verbose"),
            "Help should list verbose option"
        );

        // Test short help
        let (success, stdout, stderr) = run_drako_with_output(&["-h"]);
        assert!(success, "Short help should succeed");
        let output = format!("{}{}", stdout, stderr);
        assert!(
            output.contains("Usage:"),
            "Short help should contain usage information"
        );

        // Test version output
        let (success, stdout, stderr) = run_drako_with_output(&["--version"]);
        assert!(success, "Version command should succeed");
        let output = format!("{}{}", stdout, stderr);
        assert!(
            output.contains("drako"),
            "Version should contain program name"
        );
    }

    #[test]
    fn test_error_handling_and_recovery() {
        let temp_dir = tempdir().unwrap();

        // Test with invalid permission format
        let invalid_perm_dir = temp_dir.path().join("invalid_perm_test");
        let (success, _, stderr) = run_drako_with_output(&[
            invalid_perm_dir.to_str().unwrap(),
            "-999", // Invalid octal
            "--readme",
        ]);

        // Directory should still be created even with invalid permissions
        assert!(success, "Should succeed despite invalid permissions");
        assert!(
            invalid_perm_dir.exists(),
            "Directory should still be created"
        );
        assert!(
            invalid_perm_dir.join("README.md").exists(),
            "README should still be created"
        );

        // Test with unknown flags
        let unknown_flag_dir = temp_dir.path().join("unknown_flag_test");
        let (success, _, stderr) = run_drako_with_output(&[
            unknown_flag_dir.to_str().unwrap(),
            "--nonexistent-flag",
            "--readme",
        ]);

        assert!(success, "Should succeed despite unknown flag");
        assert!(unknown_flag_dir.exists(), "Directory should be created");
        assert!(
            unknown_flag_dir.join("README.md").exists(),
            "README should be created"
        );
        assert!(
            stderr.contains("Unknown flag"),
            "Should warn about unknown flag"
        );
    }

    #[test]
    fn test_file_content_validation() {
        let temp_dir = tempdir().unwrap();
        let content_test_dir = temp_dir.path().join("content_validation");

        let (success, _, stderr) = run_drako_with_output(&[
            content_test_dir.to_str().unwrap(),
            "--readme",
            "--license",
            "--docker",
        ]);

        assert!(
            success,
            "Content creation should succeed. stderr: {}",
            stderr
        );

        // Validate README.md content structure
        let readme_path = content_test_dir.join("README.md");
        let readme_content = fs::read_to_string(&readme_path).unwrap();

        assert!(
            readme_content.contains("# Project Title"),
            "README should have title"
        );
        assert!(
            readme_content.contains("## Description"),
            "README should have description"
        );
        assert!(
            readme_content.contains("## Getting Started"),
            "README should have getting started"
        );
        assert!(
            readme_content.contains("### Dependencies"),
            "README should have dependencies"
        );
        assert!(
            readme_content.contains("## License"),
            "README should have license section"
        );

        // Verify README has reasonable length
        assert!(
            count_file_lines(&readme_path) > 20,
            "README should have substantial content"
        );

        // Validate LICENSE content
        let license_path = content_test_dir.join("LICENSE");
        let license_content = fs::read_to_string(&license_path).unwrap();

        assert!(
            license_content.contains("MIT License"),
            "LICENSE should be MIT"
        );
        assert!(
            license_content.contains("Copyright (c)"),
            "LICENSE should have copyright"
        );
        assert!(
            license_content.contains("Permission is hereby granted"),
            "LICENSE should have permission text"
        );
        assert!(
            license_content.contains("THE SOFTWARE IS PROVIDED \"AS IS\""),
            "LICENSE should have warranty disclaimer"
        );

        // Validate Dockerfile content
        let dockerfile_path = content_test_dir.join("Dockerfile");
        let dockerfile_content = fs::read_to_string(&dockerfile_path).unwrap();

        assert!(
            dockerfile_content.contains("FROM"),
            "Dockerfile should have FROM instruction"
        );
        assert!(
            dockerfile_content.contains("WORKDIR"),
            "Dockerfile should have WORKDIR"
        );
        assert!(
            dockerfile_content.contains("COPY"),
            "Dockerfile should have COPY instruction"
        );
        assert!(
            dockerfile_content.contains("EXPOSE"),
            "Dockerfile should have EXPOSE"
        );
        assert!(
            dockerfile_content.contains("CMD"),
            "Dockerfile should have CMD"
        );
        assert!(
            dockerfile_content.contains("ARG STACK=node"),
            "Dockerfile should support multiple stacks"
        );
    }

    #[test]
    fn test_no_arguments_error_handling() {
        let (success, stdout, stderr) = run_drako_with_output(&[]);

        assert!(!success, "Should fail with no arguments");
        let output = format!("{}{}", stdout, stderr);
        assert!(
            output.contains("Usage:"),
            "Should show usage when no arguments provided"
        );
    }

    #[test]
    fn test_existing_directory_behavior() {
        let temp_dir = tempdir().unwrap();
        let existing_dir = temp_dir.path().join("existing_directory");

        // Create directory manually first
        fs::create_dir(&existing_dir).unwrap();

        let (success, stdout, stderr) =
            run_drako_with_output(&[existing_dir.to_str().unwrap(), "--readme", "--verbose"]);

        assert!(success, "Should succeed even with existing directory");
        let output = format!("{}{}", stdout, stderr);
        assert!(
            output.contains("already exists"),
            "Should warn about existing directory"
        );

        // README should not be created when directory already exists
        assert!(
            !existing_dir.join("README.md").exists(),
            "README should not be created for existing directory"
        );
    }
}
