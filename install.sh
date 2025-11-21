#!/bin/bash
set -e

# Terraform Plan Formatter installer
REPO="example/terraform-plan-formatter"
VERSION="latest"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
    darwin)
        case $ARCH in
            x86_64) TARGET="x86_64-apple-darwin" ;;
            arm64) TARGET="aarch64-apple-darwin" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    linux)
        case $ARCH in
            x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Installing terraform-plan-formatter for $TARGET..."

# Create temp directory
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

# Download and extract
if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/tfplan-$VERSION-$TARGET.tar.gz"
else
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/tfplan-$VERSION-$TARGET.tar.gz"
fi

echo "Downloading from $DOWNLOAD_URL..."
curl -L "$DOWNLOAD_URL" | tar -xz

# Install binary
sudo cp "tfplan-$VERSION-$TARGET/tfplan" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/tfplan"

# Cleanup
cd /
rm -rf "$TEMP_DIR"

echo "✓ terraform-plan-formatter installed to $INSTALL_DIR/tfplan"
echo "Run 'tfplan --help' to get started!"