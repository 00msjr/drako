pub const YELLOW: &str = "\x1b[1;33m";
pub const RED: &str = "\x1b[1;31m";
pub const GREEN: &str = "\x1b[1;32m";
pub const RESET: &str = "\x1b[0m";

pub fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    eprintln!("drako {}", VERSION);
}

pub fn usage() {
    eprintln!("{}Usage: drako new_directory [options]...", YELLOW);
}

pub fn success(input: &str, arg: Option<&str>) {
    match arg {
        Some(arg) => eprintln!("{}Success: {} {}", GREEN, input, arg),
        None => eprintln!("{}Success: {}", YELLOW, input),
    }
}

pub fn warning(input: &str, arg: Option<&str>) {
    match arg {
        Some(arg) => eprintln!("{}Warning: {} {}", YELLOW, input, arg),
        None => eprintln!("{}Warning: {}", YELLOW, input),
    }
}

pub fn error(input: &str, arg: Option<&str>) {
    match arg {
        Some(arg) => eprintln!("{}Error:{} {} {}", RED, RESET, input, arg),
        None => eprintln!("{}Error:{} {}", RED, RESET, input), // Handle the case when arg is None
    }
}

pub fn help() {
    let help_message = format!(
        "{YELLOW}Usage:{RESET} drako [directories] [options]...

{YELLOW}Help:{RESET}  Creates one or more directories with optional project initialization.
    Multiple directories can be specified, and options apply to all of them.

{YELLOW}Options:{RESET}
    {GREEN}--git,     -g{RESET}         Initialize a Git repository.
    {GREEN}--readme,  -r{RESET}         Generate a template README.md file.
    {GREEN}--license, -l{RESET}         Generate a template MIT License file.
    {GREEN}--docker,  -do{RESET}        Generate a template Docker file.
    {GREEN}--go,      -go{RESET}        Initialize a Go project.
    {GREEN}--cargo,   -c{RESET}         Initialize a Rust Cargo project.
    {GREEN}--npm,     -n{RESET}         Initialize an npm project (package.json).
    {GREEN}--bun,     -b{RESET}         Initialize a Bun project.
    {GREEN}--yarn,    -y{RESET}         Initialize a Yarn project.
    {GREEN}--pnpm,    -p{RESET}         Initialize a pnpm project.
    {GREEN}--deno,    -d{RESET}         Initialize a Deno project (deno.json).
    {GREEN}--verbose, -v{RESET}         Show detailed output from commands.
    {GREEN}           -### {RESET}      Set directory permissions (octal format, e.g., -700, -755).
    {GREEN}--help     -h{RESET}         Display this help message.
    {GREEN}--version    {RESET}         Display version.
",
    );

    eprintln!("{}", help_message);
}
