use std::env;
use std::fs;
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
    --git                 Initialize a Git repository.
    --readme              Generate a README.md file.
    --mit                 Generate an MIT License file.
    --npm                 Initialize an npm project (package.json).
    --bun                 Initialize a Bun project.
    --yarn                Initialize a Yarn project.
    --pnpm                Initialize a pnpm project.
    --cargo               Initialize a Rust Cargo project.
    --deno                Initialize a Deno project (deno.json).
";
        eprintln!("{}", usage_message);
        std::process::exit(1);
    }

    // Separate directory names from flags
    let mut dirs: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg.starts_with("--") {
            flags.push(arg.clone());
        } else {
            dirs.push(arg.clone());
        }
    }

    if dirs.is_empty() {
        eprintln!("No directories provided.");
        std::process::exit(1);
    }

    // Process each directory
    for dir in dirs {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory {}: {}", dir, e);
            continue;
        } else {
            println!("Created directory: {}", dir);
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
                if !output.status.success() {
                    eprintln!("Failed to execute: {} in {}", cmd, dir);
                }
            } else {
                eprintln!("Error running: {} in {}", cmd, dir);
            }
        };

        // Process each flag for the current directory
        for flag in &flags {
            match flag.as_str() {
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
                        eprintln!("Failed to create deno.json in {}: {}", dir, e);
                    }
                }
                "--readme" => {
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

                    if let Err(e) = fs::write(dir_path.join("README.md"), readme_content) {
                        eprintln!("Failed to create README.md in {}: {}", dir, e);
                    }
                }
                "--mit" => {
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

                    if let Err(e) = fs::write(dir_path.join("LICENSE"), license_content) {
                        eprintln!("Failed to create LICENSE file in {}: {}", dir, e);
                    }
                }
                _ => eprintln!("Unknown flag: {}", flag),
            }
        }
    }
}
