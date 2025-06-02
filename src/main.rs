mod init;
mod messages;
mod utils;

use init::*;
use messages::*;
use utils::*;

use std::env;
// use std::fs;
// use std::os::unix::fs::PermissionsExt;
// use std::path::Path;
// use std::process::Command;

fn main() {
    // cli arguments get passed to collection
    // Separate directory names from flags and permissions
    let args: Vec<String> = env::args().collect();
    let mut dirs: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    let mut permissions: Option<u32> = None;
    let mut verbose = false;
    // pub static mut VERBOSE: bool = false;

    if args.len() < 2 {
        usage();
        std::process::exit(0);
    }

    // Check for --help or -h flag
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        help();
        std::process::exit(0);
    }

    if args.contains(&"--version".to_string()) {
        version();
        std::process::exit(0);
    }

    for arg in args.iter().skip(1) {
        if !arg.starts_with('-') {
            dirs.push(arg.clone());
            continue;
        }

        match arg.as_str() {
            "--verbose" | "-v" => {
                verbose = true;
            }
            arg_str if arg_str.starts_with("--") => {
                flags.push(arg.clone());
            }
            arg_str if arg_str.starts_with('-') && arg_str.len() > 1 => {
                let perm_str = &arg_str[1..]; // Remove the dash
                if perm_str.len() <= 3 && perm_str.chars().all(|c| c.is_ascii_digit() && c < '8') {
                    match u32::from_str_radix(perm_str, 8) {
                        Ok(perm) => permissions = Some(perm),
                        Err(_) => flags.push(arg.clone()), // Treat as flag if not valid permission
                    }
                } else {
                    flags.push(arg.clone()); // Single char flags like -g, -r, etc.
                }
            }
            _ => {
                flags.push(arg.clone());
            }
        }
    }

    if dirs.is_empty() {
        error("No directories provided", None);
        std::process::exit(1);
    }

    // Process each directory
    /* fn create_directory(dir: &str, verbose: bool) -> bool {
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

    fn set_permissions(dir: &str, mode: u32, verbose: bool) {
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

    fn process_flags(dir: &str, flags: &[String], verbose: bool) {
        for flag in flags {
            match flag.as_str() {
                "--git" | "-g" => git_init(),
                "--npm" | "-n" => npm_init(),
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

    fn run_command(dir: &str, cmd: &str, verbose: bool) {
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
    } */

    for dir in &dirs {
        if create_directory(dir, verbose) {
            if let Some(mode) = permissions {
                set_permissions(dir, mode, verbose);
            }
            process_flags(dir, &flags, verbose);
        }
    }
}
