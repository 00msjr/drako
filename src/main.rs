mod files;
mod messages;
mod utils;

use messages::*;
use utils::*;

use std::env;

fn main() {
    // cli arguments get passed to collection
    // Separate directory names from flags and permissions
    let args: Vec<String> = env::args().collect();
    let mut dirs: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    let mut permissions: Option<u32> = None;
    let mut verbose = false;

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

    for dir in &dirs {
        if create_directory(dir, verbose) {
            if let Some(mode) = permissions {
                set_permissions(dir, mode, verbose);
            }
            process_flags(dir, &flags, verbose);
        }
    }
}
