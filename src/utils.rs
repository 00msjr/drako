use crate::init::*;
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
        Err(e) => {
            error("Failed to create directory", Some(dir));
            eprintln!("\x1b[1;31m{}\x1b[0m", e);
            false
        }
    }
}

pub fn set_permissions(dir: &str, mode: u32, verbose: bool) {
    if let Ok(metadata) = fs::metadata(dir) {
        let mut perms = metadata.permissions();
        perms.set_mode(mode);
        if let Err(e) = fs::set_permissions(dir, perms) {
            eprintln!(
                "\x1b[1;31mFailed to set permissions on {}:\x1b[0m {}",
                dir, e
            );
        } else if verbose {
            println!("\x1b[1;32mSet permissions {:o} on {}\x1b[0m", mode, dir);
        }
    }
}

pub fn process_flags(dir: &str, flags: &[String], verbose: bool) {
    for flag in flags {
        match flag.as_str() {
            "--git" | "-g" => git_init(),
            // "--npm" | "-n" => npm_init(),
            // "--bun" | "-b" => bun_init(),
            // "--yarn" | "-y" => yarn_init(),
            // "--pnpm" | "-p" => pnpm(),
            // "--cargo" | "-c" => cargo_init(),
            // "--go" | "-go" => run_command(dir, &format!("go mod init {}", dir)),
            // docker
            // mit
            // nix
            "--readme" => {
                let readme_path = Path::new(dir).join("README.md");
                if let Err(e) = fs::write(readme_path, "README content here") {
                    eprintln!("Failed to write README.md in {}: {}", dir, e);
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
                println!("\x1b[1;32mRan:\x1b[0m {} in {}", cmd, dir);
            }
        }
        Ok(output) => {
            eprintln!(
                "\x1b[1;31mFailed:\x1b[0m {} in {}\n{}",
                cmd,
                dir,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            eprintln!("\x1b[1;31mError running {} in {}:\x1b[0m {}", cmd, dir, e);
        }
    }
}
