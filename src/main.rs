use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let usage_message = "\
\x1b[1;33mUsage:\x1b[0m makedir [directories] [options]

\x1b[1;33mHelp:\x1b[0m  Creates one or more directories with optional project initialization.
       Multiple directories can be specified, and options apply to all of them.

\x1b[1;33mOptions:\x1b[0m
    \x1b[32m--git,     -g\x1b[0m         Initialize a Git repository.
    \x1b[32m--readme,  -r\x1b[0m         Generate a template README.md file.
    \x1b[32m--license, -l\x1b[0m         Generate a template MIT License file.
    \x1b[32m--docker,  -do\x1b[0m        Generate a template Docker file.
    \x1b[32m--go,      -go\x1b[0m        Initialize a Go project.
    \x1b[32m--cargo,   -c\x1b[0m         Initialize a Rust Cargo project.
    \x1b[32m--npm,     -n\x1b[0m         Initialize an npm project (package.json).
    \x1b[32m--bun,     -b\x1b[0m         Initialize a Bun project.
    \x1b[32m--yarn,    -y\x1b[0m         Initialize a Yarn project.
    \x1b[32m--pnpm,    -p\x1b[0m         Initialize a pnpm project.
    \x1b[32m--deno,    -d\x1b[0m         Initialize a Deno project (deno.json).
    \x1b[32m--verbose, -v\x1b[0m         Show detailed output from commands.
    \x1b[32m           -###\x1b[0m       Set directory permissions (octal format, e.g., -700, -755).
";
        eprintln!("{}", usage_message);
        std::process::exit(1);
    }

    // Separate directory names from flags and permissions
    let mut dirs: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    let mut permissions: Option<u32> = None;
    let mut verbose = false;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with("-") {
            // Check if it's a permission tag (e.g., -700)
            if arg.len() > 1 && arg.chars().nth(1).unwrap().is_numeric() {
                // Parse the permission octal value
                if let Ok(perm) = u32::from_str_radix(&arg[1..], 8) {
                    permissions = Some(perm);
                } else {
                    eprintln!("\x1b[1;31mInvalid permission format: {}\x1b[0m", arg);
                }
            }
            // Check for verbose flag
            else if arg == "--verbose" || arg == "-v" {
                verbose = true;
            }
            // It's a regular action flag
            else {
                flags.push(arg.clone());
            }
        } else {
            // Not a flag, must be a directory
            dirs.push(arg.clone());
        }

        i += 1;
    }

    if dirs.is_empty() {
        eprintln!("\x1b[1;31mNo directories provided.\x1b[0m");
        std::process::exit(1);
    }

    // Process each directory
    for dir in dirs {
        let path = Path::new(&dir);
        if path.exists() {
            println!("\x1b[1;33mDirectory already exists:\x1b[0m {}", dir);
        } else if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("\x1b[1;31mFailed to create directory {}:\x1b[0m {}", dir, e);
            continue;
        } else if verbose {
            match std::fs::canonicalize(&dir) {
                Ok(full_path) => println!(
                    "\x1b[1;33mCreating directory:\x1b[0m {}",
                    full_path.display()
                ),
                Err(_) => println!("\x1b[1;33mCreating directory:\x1b[0m {}", dir),
            }
        }

        // Set permissions if specified
        if let Some(mode) = permissions {
            match fs::metadata(&dir) {
                Ok(metadata) => {
                    let mut perms = metadata.permissions();
                    perms.set_mode(mode);
                    if let Err(e) = fs::set_permissions(&dir, perms) {
                        eprintln!(
                            "\x1b[1;31mFailed to set permissions {} on {}:\x1b[0m {}",
                            mode, dir, e
                        );
                    } else if verbose {
                        println!("\x1b[1;32mSet permissions {:o} on {}\x1b[0m", mode, dir);
                    }
                }
                Err(e) => eprintln!("\x1b[1;31mFailed to get metadata for {}:\x1b[0m {}", dir, e),
            }
        }

        let dir_path = Path::new(&dir);

        // Helper to run commands within a directory
        let run_command = |cmd: &str| {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .current_dir(dir_path)
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    if verbose {
                        let _stdout = String::from_utf8_lossy(&output.stdout);
                        println!("\x1b[1;32mSuccessfully executed:\x1b[0m {} in {}", cmd, dir);
                    }
                } else {
                    // Always show errors regardless of verbose flag
                    eprintln!(
                        "\x1b[1;31mFailed to execute:\x1b[0m {} in {} {}",
                        cmd,
                        dir,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            } else {
                eprintln!("\x1b[1;31mError running:\x1b[0m {} in {}", cmd, dir);
            }
        };

        // Process each flag for the current directory
        for flag in &flags {
            match flag.as_str() {
                "--git" | "-g" => run_command("git init"),
                "--npm" | "-n" => run_command("npm init -y"),
                "--bun" | "-b" => run_command("bun init"),
                "--yarn" | "-y" => run_command("yarn init -y"),
                "--pnpm" | "-p" => run_command("pnpm init"),
                "--cargo" | "-c" => run_command("cargo init"),
                "--go" | "-go" => run_command(&format!("go mod init {}", dir)),
                "--deno" | "-d" => {
                    if let Err(e) = fs::write(
                        dir_path.join("deno.json"),
                        "{\n  \"importMap\": \"./import_map.json\"\n}",
                    ) {
                        eprintln!(
                            "\x1b[1;31mFailed to create deno.json in {}:\x1b[0m {}",
                            dir, e
                        );
                    } else if verbose {
                        println!(
                            "\x1b[1;32mSuccessfully created deno.json in {}.\x1b[0m",
                            dir
                        );
                    }
                }
                "--docker" | "-do" => {
                    let dockerfile_content = r#"
                        # Base image (Default: Debian)
                        ARG BASE_IMAGE=debian:latest
                        FROM $BASE_IMAGE AS builder

                        # Set working directory
                        WORKDIR /app

                        # Copy project files
                        COPY . .

                        # Install dependencies based on the selected stack
                        ARG STACK=node
                        RUN case "$STACK" in \
                                node) apt update && apt install -y curl && curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && apt install -y nodejs ;; \
                                python) apt update && apt install -y python3 python3-pip ;; \
                                rust) apt update && apt install -y curl && curl https://sh.rustup.rs -sSf | sh -s -- -y ;; \
                                go) apt update && apt install -y golang ;; \
                                deno) curl -fsSL https://deno.land/install.sh | sh ;; \
                                *) echo "No valid stack specified"; exit 1 ;; \
                            esac

                        # Expose port (Modify as needed)
                        EXPOSE 3000

                        # Command to run the application (Modify based on project type)
                        CMD ["echo", "Container is running, customize CMD as needed!"]
                        "#;

                    let dockerfile_path = dir_path.join("Dockerfile");
                    if let Err(e) = fs::write(&dockerfile_path, dockerfile_content) {
                        eprintln!(
                            "\x1b[1;31mFailed to create Dockerfile in {}:\x1b[0m {}",
                            dir_path.display(),
                            e
                        );
                    } else if verbose {
                        println!(
                            "\x1b[1;32mSuccessfully created Dockerfile in {}.\x1b[0m",
                            dir_path.display()
                        );
                    }
                }
                "--readme" | "-r" => {
                    let readme_content = format!(
                        "# Project Title\n\n\
                        Simple overview of use/purpose.\n\n\
                        ## Description\n\n\
                        An in-depth paragraph about your project and overview of use.\n\n\
                        ## Getting Started\n\n\
                        ### Dependencies\n\n\
                        * Describe any prerequisites, libraries, OS version, etc., needed before installing program.\n\
                        * ex. Windows 10\n\n\
                        ### Installing\n\n\
                        * How/where to download your program\n\
                        * Any modifications needed to be made to files/folders\n\n\
                        ### Executing program\n\n\
                        * How to run the program\n\
                        * Step-by-step bullets\n\
                        ```bash\n\
                        code blocks for commands\n\
                        ```\n\n\
                        ## Help\n\n\
                        Any advice for common problems or issues.\n\
                        ```bash\n\
                        command to run if program contains helper info\n\
                        ```\n\n\
                        ## Authors\n\n\
                        Contributors names and contact info\n\
                        ex. [@soup-ms](https://github.com/soup-ms)\n\n\
                        ## Version History\n\n\
                        * v0.2.0\n\
                            * Various bug fixes and optimizations\n\
                            * See [commit change]() or See [release history]()\n\
                        * v0.1.0\n\
                            * Initial Release\n\n\
                        ## License\n\n\
                        This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details\n\n\
                        ## Acknowledgments\n\
                        https://twitter.com/dompizzie\n"
                    );

                    if let Err(e) = fs::write(dir_path.join("README.md"), &readme_content) {
                        eprintln!(
                            "\x1b[1;31mFailed to create README.md in {}:\x1b[0m {}",
                            dir, e
                        );
                    } else if verbose {
                        println!(
                            "\x1b[1;32mSuccessfully created README.md in {}.\x1b[0m",
                            dir
                        );
                    }
                }
                "--license" | "-l" => {
                    let license_content = format!(
                        "MIT License\n\n\
                        Copyright (c) [YEAR] [YOUR NAME]\n\n\
                        Permission is hereby granted, free of charge, to any person obtaining a copy\n\
                        of this software and associated documentation files (the \"Software\"), to deal\n\
                        in the Software without restriction, including without limitation the rights\n\
                        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\
                        copies of the Software, and to permit persons to whom the Software is\n\
                        furnished to do so, subject to the following conditions:\n\n\
                        The above copyright notice and this permission notice shall be included in all\n\
                        copies or substantial portions of the Software.\n\n\
                        THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\
                        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\
                        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n\
                        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n\
                        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n\
                        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n\
                        SOFTWARE.\n"
                    );

                    if let Err(e) = fs::write(dir_path.join("LICENSE"), &license_content) {
                        eprintln!(
                            "\x1b[1;31mFailed to create LICENSE file in {}:\x1b[0m {}",
                            dir, e
                        );
                    } else if verbose {
                        println!(
                            "\x1b[1;32mSuccessfully created LICENSE file in {}.\x1b[0m",
                            dir
                        );
                    }
                }
                _ => eprintln!("\x1b[1;31mUnknown flag:\x1b[0m {}", flag),
            }
        }
    }
}
