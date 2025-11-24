class Tfplan < Formula
  desc "A minimal CLI tool to format Terraform plan output in a beautiful, human-readable format"
  homepage "https://github.com/hichambouzkraoui/terraform-plan-formatter"
  url "https://github.com/hichambouzkraoui/terraform-plan-formatter/archive/v0.1.0.tar.gz"
  sha256 ""
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/tfplan", "--version"
  end
end