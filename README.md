# Terraform Plan Formatter

A minimal CLI tool to format Terraform plan output in a beautiful, human-readable format with expandable HTML output.

## Installation

### Quick Install (macOS/Linux)
```bash
curl -sSL https://raw.githubusercontent.com/example/terraform-plan-formatter/main/install.sh | bash
```

### Quick Install (Windows)
```powershell
iwr -useb https://raw.githubusercontent.com/example/terraform-plan-formatter/main/install.ps1 | iex
```

### Package Managers

#### Alpine Linux
```bash
apk add tfplan
```

### Manual Installation

#### Download Pre-built Binaries
1. Go to [Releases](https://github.com/example/terraform-plan-formatter/releases)
2. Download the appropriate binary for your platform:
   - **macOS Intel**: `tfplan-v0.1.0-x86_64-apple-darwin.tar.gz`
   - **macOS Apple Silicon**: `tfplan-v0.1.0-aarch64-apple-darwin.tar.gz`
   - **Linux**: `tfplan-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`
   - **Windows**: `tfplan-v0.1.0-x86_64-pc-windows-gnu.zip`
3. Extract and place `tfplan` in your PATH

#### Build from Source
```bash
git clone https://github.com/example/terraform-plan-formatter.git
cd terraform-plan-formatter
cargo build --release
cp target/release/tfplan /usr/local/bin/
```

## Usage

```bash
# From file
tfplan plan.json

# From stdin
terraform plan -out=plan.tfplan
terraform show -json plan.tfplan | tfplan

# Generate HTML output
tfplan --html plan.json > plan.html

# Collapsed view (headers only)
tfplan --collapsed plan.json

# Interactive mode
tfplan --interactive plan.json
```

## Features

- **Color-coded output** for different actions (create, update, delete, replace)
- **HTML output** with expandable/collapsible sections
- **Interactive mode** for real-time expand/collapse
- **Collapsed view** for overview
- **Cross-platform** support (macOS, Linux, Windows)
- **Terraform Cloud styling** for familiar look and feel

## Examples

### Terminal Output
```
▼ + aws_instance.web will be created
        ami: "ami-12345678"
        instance_type: "t3.micro"

▼ ~ aws_s3_bucket.data will be changed
        encryption: null => "AES256"
        versioning: false => true
```

### HTML Output
Interactive HTML with clickable expand/collapse functionality, dark theme, and syntax highlighting.

## Development

```bash
# Build
make build

# Test
make test

# Build for all platforms
make release

# Quick demo
make demo
```