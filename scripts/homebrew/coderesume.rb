class Coderesume < Formula
  desc "AI-powered, ATS-friendly resume generator with Typst templates"
  homepage "https://github.com/ruiyli/CodeResume"
  url "https://github.com/ruiyli/CodeResume/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_SHA256_OF_TAR.GZ"
  license "MIT"

  depends_on "rust" => :build
  depends_on "typst" => :runtime

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "crates/cr-cli"
  end

  test do
    system "#{bin}/coderesume", "--version"
  end
end
