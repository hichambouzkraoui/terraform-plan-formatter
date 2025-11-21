.PHONY: build install clean release test

# Build for current platform
build:
	cargo build --release

# Install locally
install: build
	cp target/release/tfplan /usr/local/bin/

# Clean build artifacts
clean:
	cargo clean
	rm -rf releases/

# Build for all platforms
release:
	./build.sh

# Run tests
test:
	cargo test

# Quick test with sample data
demo: build
	./target/release/tfplan sample-plan.json
	@echo "\nHTML output:"
	./target/release/tfplan --html sample-plan.json > demo.html
	@echo "Generated demo.html"

# Package for distribution
package: release
	@echo "Packages created in releases/ directory"
	@ls -la releases/