use std::fs;
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
        let test_dir = temp_dir.path().join("test-dir_with@special#chars");

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
    fn test_short_flags() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("short_flags_test");

        let status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(test_dir.to_str().unwrap())
            .arg("-r")
            .arg("-l")
            .arg("-g")
            .status()
            .unwrap();

        assert!(status.success());
        assert!(test_dir.join("README.md").exists());
        assert!(test_dir.join("LICENSE").exists());
        assert!(test_dir.join(".git").exists());
    }
}
