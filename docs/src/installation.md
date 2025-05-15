# Installation

## Prerequisites

- Rust 1.70.0 or higher
- Cargo 1.70.0 or higher

## Platforms

Rust Dataverse is compatible with the following platforms:

- Linux
- macOS (Intel and Apple Silicon)
- Windows

### Library

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
dataverse = "0.1.0" # or the latest version
```

### CLI

The CLI is installed using Cargo. To install it, run the following command:

```bash
cargo install dvcli
```

If you wish to not compile the CLI on your machine, you can also use `cargo-binstall` to install it:

```bash
cargo install cargo-binstall
cargo binstall dvcli
```

## Usage

To use the CLI, run the following command:

```bash
dvcli --help
```

This will display the help text for the CLI and help you navigate the available commands. Alternatively, you can also follow the [DVCLI](./dvcli.md) documentation.
