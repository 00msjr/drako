#!/bin/bash

set -e

# config
REPO="00msjr/drako"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="drako"

# requirements
for cmd in curl grep sed; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: $cmd is required but not installed"
    exit 1
  fi
done

# create install directory
mkdir -p "$INSTALL_DIR"

# add to PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "Adding $INSTALL_DIR to PATH"
  if [[ -n "$ZSH_VERSION" ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >>~/.zshrc
    echo "Added to ~/.zshrc"
  elif [[ -n "$BASH_VERSION" ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >>~/.bashrc
    echo "Added to ~/.bashrc"
  else
    echo 'export PATH="$HOME/.local/bin:$PATH"' >>~/.profile
    echo "Added to ~/.profile"
  fi
fi

# get latest version
echo "Fetching latest version..."
VERSION=$(curl -sSfL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$VERSION" ]; then
  echo "Error: Could not fetch latest version"
  exit 1
fi
echo "Latest version: $VERSION"

# download binary
echo "Downloading $BINARY_NAME..."
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME"
TEMP_FILE="/tmp/$BINARY_NAME"

if ! curl -sSfL -o "$TEMP_FILE" "$DOWNLOAD_URL"; then
  echo "Error: Failed to download binary"
  exit 1
fi

# install binary
echo "Installing to $INSTALL_DIR..."
mv "$TEMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# verify installation
if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
  echo "Successfully installed $BINARY_NAME!"
  echo "Restart your terminal or run 'source ~/.bashrc' to use $BINARY_NAME"
else
  echo "Error: Installation failed"
  exit 1
fi
