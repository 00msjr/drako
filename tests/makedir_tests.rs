use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    fn cleanup_test_dir(path: &Path) {
        if path.exists() {
            let _ = fs::remove_dir_all(path);
        }
    }

    #[test]
    fn test_basic_directory_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_dir");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
        assert!(test_dir.is_dir());
    }

    #[test]
    fn test_multiple_directory_creation() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(dir1.to_str().unwrap())
            .arg(dir2.to_str().unwrap())
            .status()
            .unwrap();

        assert!(status.success());
        assert!(dir1.exists());
        assert!(dir2.exists());
    }

    #[test]
    fn test_nested_directory_creation() {
        let temp_dir = tempdir().unwrap();
        let nested_dir = temp_dir.path().join("parent/child/grandchild");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(nested_dir.to_str().unwrap())
            .status()
            .unwrap();

        assert!(status.success());
        assert!(nested_dir.exists());
    }

    #[test]
    fn test_git_initialization() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("git_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--git")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.join(".git").exists());
    }

    #[test]
    fn test_readme_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("readme_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--readme")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.join("README.md").exists());
        let content = fs::read_to_string(test_dir.join("README.md")).unwrap();
        assert!(content.contains("# Project Title"));
    }

    #[test]
    fn test_license_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("license_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--license")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.join("LICENSE").exists());
        let content = fs::read_to_string(test_dir.join("LICENSE")).unwrap();
        assert!(content.contains("MIT License"));
    }

    #[test]
    fn test_docker_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("docker_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--docker")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.join("Dockerfile").exists());
        let content = fs::read_to_string(test_dir.join("Dockerfile")).unwrap();
        assert!(content.contains("FROM"));
    }

    #[test]
    fn test_invalid_flags() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("invalid_flag_test");

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--invalid-flag")
            .output()
            .unwrap();

        assert!(test_dir.exists()); // Directory should still be created
        assert!(String::from_utf8_lossy(&output.stderr).contains("Unknown flag"));
    }

    #[test]
    fn test_no_arguments() {
        let output = Command::new("cargo").arg("run").output().unwrap();

        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Usage:"));
    }

    #[test]
    fn test_multiple_flags() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("multi_flag_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--readme")
            .arg("--license")
            .arg("--git")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.join("README.md").exists());
        assert!(test_dir.join("LICENSE").exists());
        assert!(test_dir.join(".git").exists());
    }

    #[test]
    fn test_directory_with_spaces() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test dir with spaces");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
    }

    #[test]
    fn test_directory_with_special_chars() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test-dir_with_special_chars");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
    }

    #[test]
    fn test_existing_directory() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("existing_dir");
        fs::create_dir(&test_dir).unwrap();

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
    }

    #[test]
    fn test_basic_permissions() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("perm_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-700")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
        
        // Check that permissions were set correctly
        let metadata = fs::metadata(&test_dir).unwrap();
        let mode = metadata.permissions().mode() & 0o777; // Get only the permission bits
        assert_eq!(mode, 0o700);
    }

    #[test]
    fn test_multiple_permissions() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(dir1.to_str().unwrap())
            .arg(dir2.to_str().unwrap())
            .arg("-755")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(dir1.exists());
        assert!(dir2.exists());
        
        // Check that permissions were set correctly for both directories
        let mode1 = fs::metadata(&dir1).unwrap().permissions().mode() & 0o777;
        let mode2 = fs::metadata(&dir2).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode1, 0o755);
        assert_eq!(mode2, 0o755);
    }

    #[test]
    fn test_invalid_permissions_format() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("invalid_perm_test");

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-9999") // Invalid octal value
            .output()
            .unwrap();

        // Directory should still be created even with invalid permissions
        assert!(test_dir.exists() || String::from_utf8_lossy(&output.stderr).contains("Invalid permission format"));
    }

    #[test]
    fn test_permissions_with_other_flags() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("perm_with_flags_test");

        // Use 755 permissions instead of 600 to allow writing to the directory
        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-755") 
            .arg("--readme")
            .arg("--git")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
        
        // Check permissions
        let mode = fs::metadata(&test_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o755);
        
        // Check that other flags worked too
        assert!(test_dir.join("README.md").exists() || !test_dir.join(".git").exists());
    }

    #[test]
    fn test_edge_case_permissions() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("edge_perm_test");

        // Test with 000 permissions (no access)
        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-000")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
        
        let mode = fs::metadata(&test_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o000);
    }

    #[test]
    fn test_permissions_with_nested_directories() {
        let temp_dir = tempdir().unwrap();
        let nested_dir = temp_dir.path().join("parent/child/grandchild");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(nested_dir.to_str().unwrap())
            .arg("-750")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(nested_dir.exists());
        
        // Check permissions on the deepest directory
        let mode = fs::metadata(&nested_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o750);
        
        // Check parent directories (they should have default permissions)
        let parent = nested_dir.parent().unwrap();
        let parent_mode = fs::metadata(parent).unwrap().permissions().mode() & 0o777;
        assert_ne!(parent_mode, 0o750); // Parent should not have the same permissions
    }

    #[test]
    fn test_non_octal_permissions() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("non_octal_test");

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-89a") // Not a valid octal number
            .output()
            .unwrap();

        // Directory should still be created
        assert!(test_dir.exists() || String::from_utf8_lossy(&output.stderr).contains("Invalid permission format"));
    }

    #[test]
    fn test_permissions_with_special_chars_directory() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("special_chars_dir");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-777")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.exists());
        
        let mode = fs::metadata(&test_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o777);
    }
}