# Maintainer: Your Name <your.email@example.com>
pkgname=tfplan
pkgver=0.1.0
pkgrel=1
pkgdesc="Format Terraform plan output in human-readable format with HTML support"
arch=('x86_64')
url="https://github.com/example/terraform-plan-formatter"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/example/terraform-plan-formatter/archive/v$pkgver.tar.gz")
sha256sums=('REPLACE_WITH_ACTUAL_SHA256')

build() {
    cd "$srcdir/terraform-plan-formatter-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$srcdir/terraform-plan-formatter-$pkgver"
    install -Dm755 target/release/tfplan "$pkgdir/usr/bin/tfplan"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}