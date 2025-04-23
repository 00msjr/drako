#!/bin/bash

set -e

# Create .local/bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
  echo "Adding ~/.local/bin to PATH in your shell profile"
  # Determine shell and add to appropriate profile
  if [[ -n "$ZSH_VERSION" ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >>~/.zshrc
    echo "Added to ~/.zshrc. Please run 'source ~/.zshrc' after installation."
  elif [[ -n "$BASH_VERSION" ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >>~/.bashrc
    echo "Added to ~/.bashrc. Please run 'source ~/.bashrc' after installation."
  else
    echo "Please add ~/.local/bin to your PATH manually."
  fi
fi

echo "Checking for latest version..."
LATEST_VERSION=$(curl -sSfL https://api.github.com/repos/soup-ms/drako/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
echo "Latest version: $LATEST_VERSION"

echo "Downloading https://github.com/soup-ms/drako/releases/download/$LATEST_VERSION/makedir..."
curl -sSfL -o /tmp/drako "https://github.com/soup-ms/makedir/releases/download/$LATEST_VERSION/makedir"

echo "Installing to ~/.local/bin..."
mv /tmp/drako ~/.local/bin/makedir
chmod +x ~/.local/bin/drako

echo "drako was installed successfully!"
echo "You may need to restart your terminal or run 'source ~/.zshrc' (or equivalent) to use drako."

