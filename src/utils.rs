use crate::files::*;
use crate::messages::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

pub fn create_directory(dir: &str, verbose: bool) -> bool {
    let path = Path::new(dir);

    if path.exists() {
        warning("Directory already exists", Some(dir));
        return false;
    }

    match fs::create_dir_all(path) {
        Ok(_) => {
            if verbose {
                match fs::canonicalize(path) {
                    Ok(full_path) => {
                        success("Created directory", Some(&full_path.display().to_string()))
                    }
                    Err(_) => success("Created directory", Some(dir)),
                }
            }
            true
        }
        Err(_) => {
            error("Failed to create directory", Some(dir));
            false
        }
    }
}

pub fn set_permissions(dir: &str, mode: u32, verbose: bool) {
    if let Ok(metadata) = fs::metadata(dir) {
        let mut perms = metadata.permissions();
        perms.set_mode(mode);
        if fs::set_permissions(dir, perms).is_err() {
            error("Failed to set permissions on", Some(dir));
        } else if verbose {
            //TODO:
            println!("\x1b[1;32mSet permissions {:o} on {}\x1b[0m", mode, dir);
        }
    }
}

pub fn process_flags(dir: &str, flags: &[String], verbose: bool) {
    for flag in flags {
        match flag.as_str() {
            "--git" | "-g" => run_command(dir, "git init", verbose),
            "--npm" | "-n" => run_command(dir, "npm init -y", verbose),
            "--bun" | "-b" => run_command(dir, "bun init", verbose),
            "--yarn" | "-y" => run_command(dir, "yarn init -y", verbose),
            "--pnpm" | "-p" => run_command(dir, "pnpm init", verbose),
            "--cargo" | "-c" => run_command(dir, "cargo init", verbose),
            "--go" | "-go" => run_command(dir, &format!("go mod init {}", dir), verbose),
            "--readme" => {
                if create_readme(dir, verbose).is_err() {
                    error("Failed to create README.md in", Some(dir));
                }
            }
            "--docker" => {
                if create_dockerfile(dir, verbose).is_err() {
                    error("Failed to create Dockerfile in", Some(dir));
                }
            }
            "--mit" => {
                if create_mitfile(dir, verbose).is_err() {
                    error("Failed to create LICENSE in", Some(dir));
                }
            }
            _ => eprintln!("Unknown flag: {}", flag),
        }
    }
}

pub fn run_command(dir: &str, cmd: &str, verbose: bool) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .current_dir(dir)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            if verbose {
                //TODO:
                println!("\x1b[1;32mRan:\x1b[0m {} in {}", cmd, dir);
            }
        }
        Ok(output) => {
            //TODO:
            eprintln!(
                "\x1b[1;31mFailed:\x1b[0m {} in {}\n{}",
                cmd,
                dir,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            //TODO:
            eprintln!("\x1b[1;31mError running {} in {}:\x1b[0m {}", cmd, dir, e);
        }
    }
}
