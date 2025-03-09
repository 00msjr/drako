<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# makedir

[![crates.io](https://img.shields.io/crates/v/makedir?logo=rust&logoColor=white&style=flat-square)](https://crates.io/crates/makedir)

makedir is a **better directory creation tool**.

Makedir creates directories with project initialization options, so you can set up
new projects in just a few keystrokes.<br />
makedir works on all major operating systems and is written in Rust 🦀.

[Getting started](#getting-started) •
[Installation](#installation) •
[Configuration](#configuration) •
[Integrations](#third-party-integrations)

</div>

## Getting started

```sh
makedir myproject                  # Create a simple directory
makedir myproject --git            # Create a directory and initialize git
makedir myproject --git --readme   # Create a directory with git and README.md
makedir project1 project2 --npm    # Create multiple directories with npm init
makedir myproject -755             # Create a directory with permissions set to 755
makedir myproject --verbose        # Create a directory with verbose output

# Use short flags for convenience
makedir myproject -g -r            # Same as --git --readme
makedir myproject -v -755          # Create with verbose output and permissions 755
```

Create an alias for even faster usage:

```sh
# Add to your shell config file (.bashrc, .zshrc, etc.)
alias md='makedir'
alias mdv='makedir -v'             # Always use verbose output

# Then use it like this:
md newproject -g -r                # Create directory with git and README
mdv newproject -g -755             # Create with git, verbose output and permissions 755
```

## Installation

### Using Cargo

```bash
cargo install makedir
```

### Using Homebrew

```bash
brew install soup-ms/makedir/makedir
```

makedir can be installed in a few easy steps:

1. **Install binary**

   makedir runs on most major platforms. If your platform isn't listed below,
   please [open an issue](https://github.com/soup-ms/makedir/issues).

   <details>
   <summary>Linux / WSL</summary>

   > Using cargo:
   >
   > ```sh
   > cargo install makedir --locked
   > ```
   >
   > Using Homebrew:
   >
   > ```sh
   > brew install soup-ms/tap/makedir
   > ```
   >
   > Or, use the install script:
   >
   > ```sh
   > curl -sSfL https://raw.githubusercontent.com/soup-ms/makedir/master/install.sh | sh
   > ```

   </details>

   <details>
   <summary>macOS</summary>

   > Using Homebrew:
   >
   > ```sh
   > brew tap soup-ms/makedir
   > brew install makedir
   > ```
   >
   > Using cargo:
   >
   > ```sh
   > cargo install makedir --locked
   > ```
   >
   > Or, use the install script:
   >
   > ```sh
   > curl -sSfL https://raw.githubusercontent.com/soup-ms/makedir/master/install.sh | sh
   > ```

   </details>

   <details>
   <summary>Windows</summary>

   > makedir works with PowerShell, as well as shells running in Cygwin, Git
   > Bash, and MSYS2.
   >
   > The recommended way to install makedir is via cargo:
   >
   > ```sh
   > cargo install makedir --locked
   > ```
   >
   > If you're using Cygwin, Git Bash, or MSYS2, you can also use the install script:
   >
   > ```sh
   > curl -sSfL https://raw.githubusercontent.com/soup-ms/makedir/master/install.sh | sh
   > ```

   </details>

2. **Setup aliases** (optional)

   To make makedir even more convenient, add aliases to your shell configuration.

   <details>
   <summary>Bash</summary>

   > Add this to your config file (usually `~/.bashrc`):
   >
   > ```sh
   > # Basic alias
   > alias md='makedir'
   >
   > # Aliases with common options
   > alias mdg='makedir --git'
   > alias mdr='makedir --readme'
   > alias mdgr='makedir --git --readme'
   >
   > # For shell completion (if available)
   > eval "$(makedir --completion bash)"
   > ```

   </details>

   <details>
   <summary>Zsh</summary>

   > Add this to your config file (usually `~/.zshrc`):
   >
   > ```sh
   > # Basic alias
   > alias md='makedir'
   >
   > # Aliases with common options
   > alias mdg='makedir --git'
   > alias mdr='makedir --readme'
   > alias mdgr='makedir --git --readme'
   >
   > # For shell completion (if available)
   > eval "$(makedir --completion zsh)"
   > ```

   </details>

   <details>
   <summary>Fish</summary>

   > Add this to your config file (usually `~/.config/fish/config.fish`):
   >
   > ```sh
   > # Basic alias
   > alias md='makedir'
   >
   > # Aliases with common options
   > alias mdg='makedir --git'
   > alias mdr='makedir --readme'
   > alias mdgr='makedir --git --readme'
   >
   > # For shell completion (if available)
   > makedir --completion fish | source
   > ```

   </details>

   <details>
   <summary>PowerShell</summary>

   > Add this to your PowerShell profile (find it by running `echo $profile`):
   >
   > ```powershell
   > # Basic alias
   > Set-Alias -Name md -Value makedir
   >
   > # Function aliases with common options
   > function mdg { makedir --git $args }
   > function mdr { makedir --readme $args }
   > function mdgr { makedir --git --readme $args }
   > ```

   </details>

## Features

makedir provides several project initialization options:

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
alias mdweb='makedir --git --readme --npm'
alias mdrust='makedir --git --readme --cargo'
alias mdgo='makedir --git --readme --go'

# For PowerShell
function mdweb { makedir --git --readme --npm $args }
function mdrust { makedir --git --readme --cargo $args }
function mdgo { makedir --git --readme --go $args }
```

### Environment variables

Future versions of makedir may support environment variables for configuration.

## Third-party integrations

makedir can be integrated with various tools and workflows:

| Application       | Description                       | Integration                 |
| ----------------- | --------------------------------- | --------------------------- |
| Git hooks         | Automatically initialize projects | Use in post-clone hooks     |
| CI/CD pipelines   | Create project structures         | Include in workflow scripts |
| Project templates | Standardize project setup         | Combine with template tools |

## Contributing

Contributions are welcome! To contribute:

1. **Fork the repo.**
2. **Make your changes.**
3. **Test thoroughly.**
4. **Submit a Pull Request (PR).**

Feel free to open an issue for discussions or ideas.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
