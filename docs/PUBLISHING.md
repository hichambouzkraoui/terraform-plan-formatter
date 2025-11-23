# Publishing Setup Guide

This document outlines how to obtain access tokens and credentials for publishing `tfplan` to various package managers.

## GitHub Secrets Required

Add these secrets to your GitHub repository settings (`Settings > Secrets and variables > Actions`):

```
CARGO_REGISTRY_TOKEN
HOMEBREW_TOKEN
CHOCOLATEY_API_KEY
AUR_USERNAME
AUR_EMAIL
AUR_SSH_PRIVATE_KEY
SNAPCRAFT_TOKEN
ALPINE_REPO_TOKEN
```

## Package Manager Setup

### 1. Crates.io (Rust)

**Secret:** `CARGO_REGISTRY_TOKEN`

1. Create account at [crates.io](https://crates.io/)
2. Go to [Account Settings](https://crates.io/settings/tokens)
3. Click "New Token"
4. Name: `GitHub Actions - tfplan`
5. Copy the generated token

### 2. Homebrew

**Secret:** `HOMEBREW_TOKEN`

1. Create GitHub personal access token:
   - Go to GitHub Settings > Developer settings > Personal access tokens
   - Generate new token (classic)
   - Scopes: `public_repo`, `workflow`
2. Fork [homebrew-core](https://github.com/Homebrew/homebrew-core)
3. Use the personal access token

### 3. Chocolatey (Windows)

**Secret:** `CHOCOLATEY_API_KEY`

1. Create account at [chocolatey.org](https://chocolatey.org/)
2. Go to [Account](https://chocolatey.org/account)
3. Click "API Keys" tab
4. Generate new API key
5. Copy the key

### 4. AUR (Arch Linux)

**Secrets:** `AUR_USERNAME`, `AUR_EMAIL`, `AUR_SSH_PRIVATE_KEY`

1. Create account at [aur.archlinux.org](https://aur.archlinux.org/)
2. Generate SSH key pair:
   ```bash
   ssh-keygen -t ed25519 -C "your-email@example.com" -f ~/.ssh/aur
   ```
3. Add public key to AUR account:
   - Go to AUR Account Settings
   - Add `~/.ssh/aur.pub` content to SSH Public Keys
4. Set secrets:
   - `AUR_USERNAME`: Your AUR username
   - `AUR_EMAIL`: Your email
   - `AUR_SSH_PRIVATE_KEY`: Content of `~/.ssh/aur` (private key)

### 5. Snap Store

**Secret:** `SNAPCRAFT_TOKEN`

1. Create Ubuntu One account
2. Install snapcraft: `sudo snap install snapcraft --classic`
3. Login: `snapcraft login`
4. Export credentials: `snapcraft export-login --snaps=tfplan --channels=stable credentials.txt`
5. Use content of `credentials.txt` as token

### 6. Alpine Linux

**Secret:** `ALPINE_REPO_TOKEN`

1. Contact Alpine Linux maintainers via:
   - IRC: #alpine-devel on Libera.Chat
   - Mailing list: alpine-devel@lists.alpinelinux.org
2. Request contributor access
3. Obtain repository access token

## Additional Files Needed

### Chocolatey Package Spec

Create `tfplan.nuspec`:

```xml
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>tfplan</id>
    <version>0.1.0</version>
    <title>Terraform Plan Formatter</title>
    <authors>Your Name</authors>
    <description>A minimal CLI tool to format Terraform plan output</description>
    <projectUrl>https://github.com/example/terraform-plan-formatter</projectUrl>
    <licenseUrl>https://github.com/example/terraform-plan-formatter/blob/main/LICENSE</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
    <tags>terraform cli devops</tags>
  </metadata>
  <files>
    <file src="target\x86_64-pc-windows-msvc\release\tfplan.exe" target="tools\tfplan.exe" />
  </files>
</package>
```

### AUR PKGBUILD

Create `PKGBUILD`:

```bash
pkgname=tfplan
pkgver=0.1.0
pkgrel=1
pkgdesc="A minimal CLI tool to format Terraform plan output"
arch=('x86_64')
url="https://github.com/example/terraform-plan-formatter"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/terraform-plan-formatter-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$srcdir/terraform-plan-formatter-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
```

### Snap Configuration

Create `snap/snapcraft.yaml`:

```yaml
name: tfplan
version: '0.1.0'
summary: Terraform Plan Formatter
description: A minimal CLI tool to format Terraform plan output in a beautiful, human-readable format

base: core22
confinement: strict
grade: stable

apps:
  tfplan:
    command: bin/tfplan
    plugs: [home, removable-media]

parts:
  tfplan:
    plugin: rust
    source: .
    build-packages:
      - build-essential
```

## Security Best Practices

1. **Rotate tokens regularly** (every 6-12 months)
2. **Use minimal permissions** for each token
3. **Monitor token usage** in respective platforms
4. **Revoke unused tokens** immediately
5. **Use environment-specific tokens** when possible

## Troubleshooting

### Common Issues

1. **Token expired**: Regenerate and update GitHub secret
2. **Permission denied**: Verify token has correct scopes
3. **Package exists**: Increment version number
4. **Build failures**: Check platform-specific requirements

### Support Contacts

- **Crates.io**: [help@crates.io](mailto:help@crates.io)
- **Homebrew**: GitHub issues on homebrew-core
- **Chocolatey**: [support@chocolatey.org](mailto:support@chocolatey.org)
- **AUR**: aur-general@archlinux.org
- **Snap**: [forum.snapcraft.io](https://forum.snapcraft.io)
- **Alpine**: alpine-devel@lists.alpinelinux.org