# Dataverse Rust

This is a Rust client for the Dataverse API and includes a command line interface. It is a work in progress and is not yet feature complete.

Current features include:

* Collection
  * `create`
  * `delete`
  * `publish`
* Info
  * `version`
* Dataset
  * `get`
  * `create`
  * `edit`
  * `delete`
  * `publish`
  * `link`
* File
  * `download`
  * `upload`
  * `replace`

## Installation

**Command line**

```bash
cargo install --path .
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
Dataverse Version: 6.2
```

## Examples

We have provided an example in the `examples` directory. These examples demonstrate how to use the client to perform various operations.

* [`examples/create-upload-publish`](examples/create-upload-publish) - Demonstrates how to create a collection, dataset, upload a file, and publish the collection and dataset.

## ToDo's

- [ ] Implement remaining API endpoints
- [ ] Write unit and integration tests
- [ ] Asynchronous support using `tokio`
- [ ] Local storage of credentials using `keyring` crate
- [ ] Python bindings using `PyO3`
- [ ] Documentation
- [ ] Publish on crates.io
- [ ] Continuous integration
- [ ] Validate before upload using `/api/dataverses/$ID/validateDatasetJson`
