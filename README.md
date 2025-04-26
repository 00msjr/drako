<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# drako

[![crates.io](https://img.shields.io/crates/v/drako?logo=rust&logoColor=white&style=flat-square)](https://crates.io/crates/drako)

drako is a **better directory creation tool**.

drako creates directories with project initialization options, so you can set up
new projects in just a few keystrokes.<br />
drako works on all major operating systems and is written in Rust 🦀.

[Getting started](#getting-started) •
[Installation](#installation) •
[Configuration](#configuration) •
[Integrations](#third-party-integrations)

</div>

## Getting started

```sh
drako myproject                  # Create a simple directory
drako myproject --git            # Create a directory and initialize git
drako myproject --git --readme   # Create a directory with git and README.md
drako project1 project2 --npm    # Create multiple directories with npm init
drako myproject -755             # Create a directory with permissions set to 755
drako myproject --verbose        # Create a directory with verbose output

# Use short flags for convenience
drako myproject -g -r            # Same as --git --readme
drako myproject -v -755          # Create with verbose output and permissions 755
```

Create an alias for even faster usage:

```sh
# Add to your shell config file (.bashrc, .zshrc, etc.)
alias dr='drako'
alias drv='drako -v'             # Always use verbose output

# Then use it like this:
dr newproject -g -r                # Create directory with git and README
drv newproject -g -755             # Create with git, verbose output and permissions 755
```

## Installation

### Using Cargo

```bash
cargo install drako
```

### Using Homebrew

```bash
brew install 00msjr/tap/drako
```

drako can be installed in a few easy steps:

1. **Install binary**

   drako runs on most major platforms. If your platform isn't listed below,
   please [open an issue](https://github.com/00msjr/drako/issues).

      <details>
      <summary>Linux / WSL</summary>

   > Using cargo:
   >
   > ```sh
   > cargo install drako --locked
   > ```
   >
   > Using Homebrew:
   >
   > ```sh
   > brew install 00msjr/tap/drako
   > ```
   >
   > Or, use the install script:
   >
   > ```sh
   > curl -sSfL https://raw.githubusercontent.com/00msjr/drako/master/install.sh | sh
   > ```

      </details>

      <details>
      <summary>macOS</summary>

   > Using Homebrew:
   >
   > ```sh
   > brew tap 00msjr/drako
   > brew install drako
   > ```
   >
   > Using cargo:
   >
   > ```sh
   > cargo install drako --locked
   > ```
   >
   > Or, use the install script:
   >
   > ```sh
   > curl -sSfL https://raw.githubusercontent.com/00msjr/drako/master/install.sh | sh
   > ```

      </details>

      <details>
      <summary>Windows</summary>

   > drako works with PowerShell, as well as shells running in Cygwin, Git
   > Bash, and MSYS2.
   >
   > The recommended way to install drako is via cargo:
   >
   > ```sh
   > cargo install drako --locked
   > ```
   >
   > If you're using Cygwin, Git Bash, or MSYS2, you can also use the install script:
   >
   > ```sh
   > curl -sSfL https://raw.githubusercontent.com/00msjr/drako/master/install.sh | sh
   > ```

      </details>

2. **Setup aliases** (optional)

   To make drako even more convenient, add aliases to your shell configuration.

   <details>
   <summary>Bash</summary>

   > Add this to your config file (usually `~/.bashrc`):
   >
   > ```sh
   > # Basic alias
   > alias md='drako'
   >
   > # Aliases with common options
   > alias mdg='drako --git'
   > alias mdr='drako --readme'
   > alias mdgr='drako --git --readme'
   >
   > # For shell completion (if available)
   > eval "$(drako --completion bash)"
   > ```

   </details>

   <details>
   <summary>Zsh</summary>

   > Add this to your config file (usually `~/.zshrc`):
   >
   > ```sh
   > # Basic alias
   > alias md='drako'
   >
   > # Aliases with common options
   > alias mdg='drako --git'
   > alias mdr='drako --readme'
   > alias mdgr='drako --git --readme'
   >
   > # For shell completion (if available)
   > eval "$(drako --completion zsh)"
   > ```

   </details>

   <details>
   <summary>Fish</summary>

   > Add this to your config file (usually `~/.config/fish/config.fish`):
   >
   > ```sh
   > # Basic alias
   > alias md='drako'
   >
   > # Aliases with common options
   > alias mdg='drako --git'
   > alias mdr='drako --readme'
   > alias mdgr='drako --git --readme'
   >
   > # For shell completion (if available)
   > drako --completion fish | source
   > ```

   </details>

   <details>
   <summary>PowerShell</summary>

   > Add this to your PowerShell profile (find it by running `echo $profile`):
   >
   > ```powershell
   > # Basic alias
   > Set-Alias -Name md -Value drako
   >
   > # Function aliases with common options
   > function mdg { drako --git $args }
   > function mdr { drako --readme $args }
   > function mdgr { drako --git --readme $args }
   > ```

   </details>

## Features

drako provides several project initialization options:

| Flag        | Short  | Description                                                |
| ----------- | ------ | ---------------------------------------------------------- |
| `--git`     | `-g`   | Initialize a Git repository                                |
| `--readme`  | `-r`   | Generate a template README.md file                         |
| `--license` | `-l`   | Generate a template MIT License file                       |
| `--docker`  | `-do`  | Generate a template Docker file                            |
| `--go`      | `-go`  | Initialize a Go project                                    |
| `--cargo`   | `-c`   | Initialize a Rust Cargo project                            |
| `--npm`     | `-n`   | Initialize an npm project (package.json)                   |
| `--bun`     | `-b`   | Initialize a Bun project                                   |
| `--yarn`    | `-y`   | Initialize a Yarn project                                  |
| `--pnpm`    | `-p`   | Initialize a pnpm project                                  |
| `--deno`    | `-d`   | Initialize a Deno project (deno.json)                      |
| `--verbose` | `-v`   | Show detailed output from commands                         |
|             | `-###` | Set directory permissions (octal format, e.g., -700, -755) |

## Configuration

### Custom aliases

You can create custom aliases with your most frequently used options:

```sh
# For bash/zsh/fish
alias mdweb='drako --git --readme --npm'
alias mdrust='drako --git --readme --cargo'
alias mdgo='drako --git --readme --go'

# For PowerShell
function mdweb { drako --git --readme --npm $args }
function mdrust { drako --git --readme --cargo $args }
function mdgo { drako --git --readme --go $args }
```

### Environment variables

Future versions of drako may support environment variables for configuration.

## Third-party integrations

drako can be integrated with various tools and workflows:

| Application       | Description                       | Integration                 |
| ----------------- | --------------------------------- | --------------------------- |
| Git hooks         | Automatically initialize projects | Use in post-clone hooks     |
| CI/CD pipelines   | Create project structures         | Include in workflow scripts |
| Project templates | Standardize project setup         | Combine with template tools |

## Roadmap

The following items outline our development plan for evolving drako into a more powerful directory creation and management tool:

### Version 1.0.0 Roadmap

- [x] Rename project from "makedir" to "drako" for better branding
- [x] Code optimization and refactoring
  - [ ] Separate functions into individual files for better maintainability
  - [ ] Implement proper module imports for features (-l, -r, -do, etc.)
  - [ ] Standardize code formatting across the codebase
- [ ] Command improvements
  - [ ] Add comprehensive `--help` documentation
  - [ ] Implement `--cd` flag to automatically change into created directory
  - [ ] Support `mkdir -p "$1" && cd "$1"` pattern for nested directory creation
- [ ] Feature enhancements
  - [ ] Add directory tree creation (e.g., `drako testdir(src, public, tests)`)
  - [ ] Implement password protection for directories
- [ ] Distribution
  - [ ] Submit PR to homebrew-core for official distribution
  - [ ] Reorganize project structure with homebrew file in main directory

## Contributing

Contributions are welcome! To contribute:

1. **Fork the repo.**
2. **Make your changes.**
3. **Test thoroughly.**
4. **Submit a Pull Request (PR).**

Feel free to open an issue for discussions or ideas.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
