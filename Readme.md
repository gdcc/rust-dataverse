<div align="center">
  <img src="./static/image.png" width="190">
</div>

# Dataverse Rust

![Build Status](https://github.com/JR-1991/rust-dataverse/actions/workflows/tests.yml/badge.svg)

**Dataverse Rust** is a client library and command-line interface (CLI) for interacting with
the [Dataverse API](https://guides.dataverse.org/en/latest/api/). This project is in active development and not yet
feature complete.

## Features

Current capabilities include:

### Collection Management

- **Create**: Create a new collection within the Dataverse.
- **Delete**: Remove an existing collection.
- **Publish**: Publish a collection to make it publicly available.
- **Contents**: Retrieve the contents of a collection.

### General Information

- **Version**: Retrieve the current version of the Dataverse instance.

### Dataset Management

- **Get**: Fetch details of a specific dataset.
- **Create**: Create a new dataset within a collection.
- **Edit**: Modify an existing dataset.
- **Delete**: Delete an unpublished dataset.
- **Upload**: Upload a file to a dataset.
- **Publish**: Publish a dataset to make it publicly available.
- **Link**: Link datasets to other collections.

### File Management

- **Replace**: Replace existing files in a dataset.

## Installation

**Command line**

To install the command line tool, use `cargo:

```bash
cargo install --git https://github.com/JR-1991/rust-dataverse.git
```

After installation, you can interact with the Dataverse API using the dvcli command. Note that this command installs only the CLI tool, not the library itself. Instructions for adding the library to your project are provided below.

**Library**

To include the library in your project, add this to your Cargo.toml file:

```toml
[dependencies]
dataverse = { git = "https://github.com/JR-1991/rust-dataverse" }
```

Since the library isn’t published on [crates.io](https://crates.io) yet, we’re using a git dependency. Once added, you can begin using the library in your project.

## Usage

### Command line

Before you can use the command line tool, you need to set the `DVCLI_URL` and `DVCLI_TOKEN` environment variables. You
can do this by adding the following lines to your `.bashrc` or `.bash_profile` file:

```bash
export DVCLI_URL="https://your.dataverse.url"
export DVCLI_TOKEN="your_token_here"
```

The command line tool in organized in subcommands. To see a list of available subcommands, run:

```bash
dvcli --help
```

To see help for a specific subcommand, run:

```bash
dvcli <subcommand> --help
```

**Example**

In this examples we will demonstrate how to retrieve the version of the Dataverse instance.

```bash
dvcli info version
```

The output will be similar to:

```bash
Calling: http://localhost:8080/api/info/version
└──  🎉 Success! - Received the following response:

{
  "version": "6.2"
}
```

## Examples

We have provided an example in the `examples` directory. These examples demonstrate how to use the client to perform
various operations.

* [`examples/create-upload-publish`](examples/create-upload-publish) - Demonstrates how to create a collection, dataset,
  upload a file, and publish the collection and dataset.

## ToDo's

- [ ] Implement remaining API endpoints
- [x] Write unit and integration tests
- [x] Asynchronous support using `tokio`
- [x] Documentation
- [ ] Publish on crates.io
- [x] Continuous integration
- [ ] Validate before upload using `/api/dataverses/$ID/validateDatasetJson`
