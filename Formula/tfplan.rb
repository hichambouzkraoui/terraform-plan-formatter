class Tfplan < Formula
  desc "Format Terraform plan output in human-readable format with HTML support"
  homepage "https://github.com/example/terraform-plan-formatter"
  version "0.1.0"
  
  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/example/terraform-plan-formatter/releases/download/v0.1.0/tfplan-v0.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/example/terraform-plan-formatter/releases/download/v0.1.0/tfplan-v0.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    url "https://github.com/example/terraform-plan-formatter/releases/download/v0.1.0/tfplan-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "tfplan"
  end

  test do
    system "#{bin}/tfplan", "--help"
  end
end