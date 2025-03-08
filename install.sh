#!/bin/sh
# makedir installer
#
# This is just a little script that can be downloaded from the internet to
# install makedir. It just does platform detection, downloads the latest binary
# version and installs it in the appropriate location.
#
# Heavily inspired by the Rustup and Deno installers

set -e

RELEASES_URL="https://github.com/soup-ms/makedir/releases"
GITHUB_REPO="soup-ms/makedir"

main() {
    need_cmd curl
    need_cmd grep
    need_cmd head
    need_cmd tail
    need_cmd cut
    need_cmd tr
    need_cmd uname

    get_architecture || return 1
    
    # Parse command line arguments
    parse_args "$@"
    
    # Show what will be installed if in dry run mode
    if [ "$DRY_RUN" = true ]; then
        echo "Would download and install makedir for $TARGET"
        return 0
    fi

    # Download and install the binary
    download_and_install || return 1

    echo "makedir was installed successfully!"
}

parse_args() {
    DRY_RUN=false
    
    while [ $# -gt 0 ]; do
        case "$1" in
            --dry-run)
                DRY_RUN=true
                ;;
            *)
                echo "Unknown option: $1"
                echo "Usage: install.sh [--dry-run]"
                exit 1
                ;;
        esac
        shift
    done
}

get_architecture() {
    local _ostype _cputype

    _ostype="$(uname -s)"
    _cputype="$(uname -m)"

    case "$_ostype" in
        Linux)
            _ostype=linux
            ;;
        Darwin)
            _ostype=darwin
            ;;
        MINGW* | MSYS* | CYGWIN*)
            _ostype=windows
            ;;
        *)
            err "unsupported OS type: $_ostype"
            ;;
    esac

    case "$_cputype" in
        x86_64 | x86-64 | x64 | amd64)
            _cputype=x86_64
            ;;
        aarch64 | arm64)
            _cputype=aarch64
            ;;
        *)
            err "unsupported CPU type: $_cputype"
            ;;
    esac

    TARGET="${_cputype}-${_ostype}"
}

download_and_install() {
    # Determine latest version
    echo "Checking for latest version..."
    VERSION=$(curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | 
              grep '"tag_name":' | 
              head -n 1 | 
              cut -d '"' -f 4)
    
    if [ -z "$VERSION" ]; then
        err "Could not determine latest version"
        return 1
    fi
    
    echo "Latest version: $VERSION"
    
    # Create temp directory
    TMP_DIR=$(mktemp -d)
    
    # Download binary
    ARCHIVE="makedir-${TARGET}.tar.gz"
    URL="$RELEASES_URL/download/$VERSION/$ARCHIVE"
    
    echo "Downloading $URL..."
    curl -L --proto '=https' --tlsv1.2 -sSf "$URL" -o "$TMP_DIR/$ARCHIVE"
    
    # Extract and install
    echo "Extracting..."
    tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"
    
    # Determine install location
    INSTALL_DIR="/usr/local/bin"
    if [ ! -w "$INSTALL_DIR" ]; then
        # Try to use $HOME/.local/bin if /usr/local/bin is not writable
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi
    
    echo "Installing to $INSTALL_DIR..."
    mv "$TMP_DIR/makedir" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/makedir"
    
    # Clean up
    rm -rf "$TMP_DIR"
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "need '$1' (command not found)"
        exit 1
    fi
}

err() {
    echo "Error: $1" >&2
    exit 1
}

main "$@"