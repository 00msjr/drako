use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: makedir <directory> [--git] [--npm] [--bun] [--yarn] [--pnpm] [--cargo] [--deno] [--readme] [--mitlicense]");
        std::process::exit(1);
    }

    let dir_name = &args[1];

    // Create the directory
    if let Err(e) = fs::create_dir_all(dir_name) {
        eprintln!("Failed to create directory: {}", e);
        std::process::exit(1);
    } else {
        println!("Created directory: {}", dir_name);
    }

    let dir_path = std::path::Path::new(dir_name);

    // Helper function to run commands inside the directory
    let run_command = |cmd: &str| {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(dir_path)
            .output();

        if let Ok(output) = output {
            if !output.status.success() {
                eprintln!("Failed to execute: {}", cmd);
            }
        } else {
            eprintln!("Error running: {}", cmd);
        }
    };

    // Process flags
    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--git" => run_command("git init"),
            "--npm" => run_command("npm init -y"),
            "--bun" => run_command("bun init"),
            "--yarn" => run_command("yarn init -y"),
            "--pnpm" => run_command("pnpm init"),
            "--cargo" => run_command("cargo init"),
            "--deno" => {
                if let Err(e) = fs::write(
                    dir_path.join("deno.json"),
                    "{\n  \"importMap\": \"./import_map.json\"\n}",
                ) {
                    eprintln!("Failed to create deno.json: {}", e);
                }
            }
            "--readme" => {
                if let Err(e) = fs::write(dir_path.join("README.md"), format!("# {}\n", dir_name)) {
                    eprintln!("Failed to create README.md: {}", e);
                }
            }
            "--mitlicense" => {
                if let Err(e) = fs::write(dir_path.join("LICENSE"), "MIT License") {
                    eprintln!("Failed to create LICENSE: {}", e);
                }
            }
            _ => eprintln!("Unknown flag: {}", arg),
        }
    }
}
