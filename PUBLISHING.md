# CodeResume Publishing Guide

This guide covers publishing CodeResume to crates.io and Homebrew.

## Overview

CodeResume is a **CLI binary** project with internal library crates. Only the `cr-cli` binary crate is published to crates.io for end users. Internal library crates (`cr-core`, `cr-io`, `cr-ai`, `cr-render`, `cr-service`, `cr-config`) are marked with `publish = false` and not published separately.

## Publishing to crates.io

### Prerequisites

1. Create an account at https://crates.io
2. Generate an API token: https://crates.io/me
3. Authenticate locally:
   ```bash
   cargo login
   # Paste your API token when prompted
   ```

### First-Time Publication (v0.1.0)

For the first release, internal crates must be published first to satisfy dependency resolution:

#### Step 1: Temporarily enable publishing for library crates
```bash
# Edit each crate's Cargo.toml and remove or comment out `publish = false`
# Or use find+replace in your editor
```

#### Step 2: Publish in dependency order
```bash
# cr-core (has no internal dependencies)
cargo publish -p cr-core

# Wait for crates.io to sync (usually <1 minute)

# Dependencies of cr-core
cargo publish -p cr-config
cargo publish -p cr-io

# Dependencies of the above
cargo publish -p cr-ai
cargo publish -p cr-render

# All remaining internal crates
cargo publish -p cr-service

# Finally, the CLI binary
cargo publish -p cr-cli
```

#### Step 3: Restore publish = false
```bash
# Re-add `publish = false` to all library crates
```

### Subsequent Releases

After the first v0.1.0 publication:

1. **Update version** in `Cargo.toml` (workspace level):
   ```toml
   [workspace.package]
   version = "0.2.0"  # Update this
   ```

2. **Update internal crate versions** in `crates/cr-cli/Cargo.toml`:
   ```toml
   cr-ai = { path = "../cr-ai", version = "0.2" }
   # etc.
   ```

3. **Test with dry-run** (may fail for internal crates due to publish=false):
   ```bash
   cargo publish --dry-run -p cr-cli --allow-dirty
   ```

4. **Publish**:
   ```bash
   cargo publish -p cr-cli
   ```

### Troubleshooting

**Error: "no matching package named `cr-ai` found"**
- This happens when internal crates aren't on crates.io yet
- Solutions:
  - Use `--allow-dirty` for testing: `cargo publish --dry-run --allow-dirty -p cr-cli`
  - Temporarily remove `publish = false` to publish all crates
  - Ensure all internal crates are published before publishing cr-cli

**Error: "failed to verify manifest"**
- Dependencies must have version requirements specified
- Check that `Cargo.toml` includes versions: `cr-ai = { path = "../cr-ai", version = "0.1" }`

## Publishing to Homebrew

### Create a Homebrew Formula

1. Create file: `scripts/homebrew/coderesume.rb`

```ruby
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
