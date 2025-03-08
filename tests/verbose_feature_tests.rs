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
    fn test_verbose_flag_basic() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        
        // Check that verbose output contains expected information
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Creating directory"));
    }

    #[test]
    fn test_verbose_with_nested_directories() {
        let temp_dir = tempdir().unwrap();
        let nested_dir = temp_dir.path().join("parent/child/grandchild");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&nested_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(nested_dir.to_str().unwrap())
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(nested_dir.exists());
        
        // Check that verbose output mentions the directory
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Creating directory"));
    }

    #[test]
    fn test_verbose_with_multiple_directories() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");

        // Make sure the directories don't exist
        cleanup_test_dir(&dir1);
        cleanup_test_dir(&dir2);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(dir1.to_str().unwrap())
            .arg(dir2.to_str().unwrap())
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(dir1.exists());
        assert!(dir2.exists());
        
        // Check that verbose output mentions directories
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Creating directory"));
    }

    #[test]
    fn test_verbose_with_permissions() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_perm_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-755")
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        
        // Check permissions were set correctly
        let mode = fs::metadata(&test_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o755);
        
        // Check that verbose output mentions permissions
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("755"));
    }

    #[test]
    fn test_verbose_with_readme_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_readme_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--readme")
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        assert!(test_dir.join("README.md").exists());
        
        // Check that verbose output mentions README creation
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("README.md"));
    }

    #[test]
    fn test_verbose_with_license_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_license_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--license")
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        assert!(test_dir.join("LICENSE").exists());
        
        // Check that verbose output mentions LICENSE creation
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("LICENSE"));
    }

    #[test]
    fn test_verbose_with_docker_creation() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_docker_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--docker")
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        assert!(test_dir.join("Dockerfile").exists());
        
        // Check that verbose output mentions Dockerfile creation
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Dockerfile"));
    }

    #[test]
    fn test_verbose_short_flag() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_short_flag_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-v")  // Short flag version
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        
        // Check that verbose output works with short flag too
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Creating directory"));
    }

    #[test]
    fn test_verbose_with_existing_directory() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_existing_dir");
        
        // Create the directory first
        fs::create_dir(&test_dir).unwrap();

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        
        // Check that verbose output mentions directory already exists
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("already exists"));
    }

    #[test]
    fn test_verbose_with_git_initialization() {
        // Skip this test if git is not installed
        let git_check = Command::new("which")
            .arg("git")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if !git_check {
            println!("Skipping git test as git command is not available");
            return;
        }

        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("verbose_git_test");

        // Make sure the directory doesn't exist
        cleanup_test_dir(&test_dir);

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("--git")
            .arg("--verbose")
            .output()
            .unwrap();

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
        assert!(test_dir.exists());
        
        // Check that verbose output mentions git initialization
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("git") || stdout.contains("Git"));
    }
}