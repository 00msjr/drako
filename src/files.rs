use crate::messages::*;
use std::fs;
use std::path::Path;

pub fn create_readme(dir: &str, verbose: bool) -> Result<(), std::io::Error> {
    let content = readme_content();
    let path = Path::new(dir).join("README.md");
    fs::write(path, content)?;

    if verbose {
        success("Created README.md in", Some(dir));
    }
    Ok(())
}

pub fn create_mitfile(dir: &str, verbose: bool) -> Result<(), std::io::Error> {
    let content = mit_content();
    let path = Path::new(dir).join("LICENSE");
    fs::write(path, content)?;

    if verbose {
        success("Created LICENSE in", Some(dir));
    }
    Ok(())
}

pub fn create_dockerfile(dir: &str, verbose: bool) -> Result<(), std::io::Error> {
    let content = docker_content();
    let path = Path::new(dir).join("Dockerfile");
    fs::write(path, content)?;

    if verbose {
        success("Created Dockerfile in", Some(dir));
    }
    Ok(())
}

pub fn readme_content() -> String {
    format!(
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
        ex. [@00msjr](https://github.com/soup-ms)\n\n\
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
    )
}

pub fn docker_content() -> String {
    r#"# Base image (Default: Debian)
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
"#.to_string()
}

pub fn mit_content() -> String {
    format!(
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
    )
}
