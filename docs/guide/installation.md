# Installation

## npm (recommended)

```bash
npm install -g miskin
```

Downloads the pre-built binary for your platform. No Rust required.

## Cargo

```bash
cargo install miskin
# or from source:
cargo install --git https://github.com/sonyarianto/miskin
```

## From Source

```bash
git clone https://github.com/sonyarianto/miskin
cd miskin
cargo install --path .
```

## Pre-built Binaries

Binaries are built via GitHub Actions on every tagged release. Download from [GitHub Releases](https://github.com/sonyarianto/miskin/releases):

- `miskin-x86_64-unknown-linux-gnu.tar.gz`
- `miskin-x86_64-unknown-linux-musl.tar.gz`
- `miskin-x86_64-apple-darwin.tar.gz`
- `miskin-aarch64-apple-darwin.tar.gz`
- `miskin-x86_64-pc-windows-msvc.zip`

### Linux/macOS

```bash
tar xzf miskin-x86_64-unknown-linux-gnu.tar.gz
sudo mv miskin /usr/local/bin/
```

### Windows

Extract the zip and add `miskin.exe` to your `PATH`.

## Verify

```bash
miskin --version
```

## Requirements

- **Rust** (for cargo install / from-source builds): [rustup.rs](https://rustup.rs)
- **Pre-built binaries**: no dependencies — single static binary
