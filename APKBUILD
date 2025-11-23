# Maintainer: Your Name <your.email@example.com>
pkgname=tfplan
pkgver=0.1.0
pkgrel=0
pkgdesc="A minimal CLI tool to format Terraform plan output"
url="https://github.com/example/terraform-plan-formatter"
arch="all"
license="MIT"
makedepends="cargo"
source="$pkgname-$pkgver.tar.gz::https://github.com/example/terraform-plan-formatter/archive/v$pkgver.tar.gz"
builddir="$srcdir/terraform-plan-formatter-$pkgver"

build() {
	cargo build --release --locked
}

check() {
	cargo test --release --locked
}

package() {
	install -Dm755 target/release/tfplan "$pkgdir"/usr/bin/tfplan
}

sha512sums="SKIP"