# Dataverse Rust

This is a Rust client for the Dataverse API. It is a work in progress and is not yet feature complete.

## Installation

**Command line**

```bash
cargo build --bin dvcli --release
```

**Cargo.toml**

Please note, this crate is not yet published on crates.io. You can add it to your `Cargo.toml` file by pointing to the GitHub repository.

```toml
[dependencies]
dataverse = { git = "https://github.com/JR-1991/rust-dataverse" }
```

## Usage

### Command line

Before you can use the command line tool, you need to set the `DVCLI_URL` and `DVCLI_TOKEN` environment variables. You can do this by adding the following lines to your `.bashrc` or `.bash_profile` file:

```bash
export DVCLI_URL="https://dataverse.harvard.edu"
export DVCLI_TOKEN="your_token_here"
```

The command line tool in organized in subcommands. To see a list of available subcommands, run:

```bash
./dvcli --help
```

To see help for a specific subcommand, run:

```bash
./dvcli <subcommand> --help
```

**Example**

In this examples we will demonstrate how to retrieve the version of the Dataverse instance.

```bash
dvcli info version
```

The output will be similar to:

```bash
Dataverse Version: 6.2
```

### Library Example

The library part of this crate refelects all functions that are available in the command line tool. The following example demonstrates how to create a collection.

```rust
use dataverse::client::DataverseClient;
use dataverse::native_api
use dataverse::models::collection

fn main() {
    let base_url = std::env::var("DVCLI_URL").unwrap();
    let token = std::env::var("DVCLI_TOKEN").unwrap();
    let client = DataverseClient::new(&base_url, &token);

    // First build the request body
    let body = collection::CreateBody::new(
        "name",
        "alias",
        "affiliation",
        "description",
        collection::DataverseType::RESEARCH_GROUP,
    )

    body.add_contact("john@doe.com");

    // Alternatively, you can also initialize the body from YAML
    let body = collection::CreateBody::from_yaml("path/to/file.yaml");

    // Perform the request
    let response = native_api::collection::create(&client, &body);

    match response {
        Ok(collection) => println!("Collection created: {}", collection.id),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
