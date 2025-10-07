<div align="center">
  <img src="./static/image.png" width="190">
</div>

# Dataverse Rust

![Build Status](https://github.com/JR-1991/rust-dataverse/actions/workflows/tests.yml/badge.svg)

A comprehensive Rust library and command-line interface for interacting with the [Dataverse API](https://guides.dataverse.org/en/latest/api/). Build robust data repository workflows with type-safe, asynchronous operations.

> **Note:** This project is under active development. While core functionality is stable, the API may evolve before the 1.0 release.

## Why Dataverse Rust?

- **🚀 High Performance** - Built with async/await using Tokio and Reqwest for efficient concurrent operations
- **🔒 Type Safety** - Leverage Rust's type system to catch errors at compile time
- **⚡ Direct Upload** - Parallel batch uploads for fast file transfers to S3-compatible storage
- **🎯 Dual Interface** - Use as a library in your Rust projects or as a standalone CLI tool
- **🔐 Secure Authentication** - Multiple auth methods including system keyring integration for credential storage
- **📦 Flexible Configuration** - JSON and YAML support for all configuration files

## Features

**Dataverse Rust** provides complete coverage of core Dataverse operations through both a programmatic library interface and a full-featured CLI:

### 📚 Collections

Create, publish, and manage Dataverse collections with hierarchical organization support.

### 📊 Datasets

Full dataset lifecycle management including creation, metadata editing, versioning, publishing, linking, and deletion. Support for dataset locks and review workflows.

### 📁 Files

Upload files via standard or direct upload (with parallel batch support), replace existing files, download files and complete datasets, and manage file metadata.

### 🔍 Search

Query datasets and files across your Dataverse instance with flexible search parameters.

### 🛠️ Administration

Manage storage drivers, configure external tools, and perform administrative operations.

### ℹ️ Instance Information

Retrieve version information and available metadata exporters from your Dataverse instance.

## Installation

### CLI Installation

Install the command-line tool directly from the repository:

```bash
cargo install --git https://github.com/JR-1991/rust-dataverse.git
```

### Library Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dataverse = { git = "https://github.com/JR-1991/rust-dataverse" }
```

> **Note:** Not yet published on crates.io. Pre-1.0 releases will be available soon.

## Usage

### Library Usage

The library provides an async API built on `tokio` and `reqwest`. Import the prelude for common types:

```rust
use dataverse::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = BaseClient::new(
        "https://demo.dataverse.org",
        Some("your-api-token")
    )?;
    
    // Get instance version
    let version = info::get_version(&client).await?;
    println!("Dataverse version: {}", version.data.unwrap());
    
    // Create a dataset
    let dataset_body = dataset::create::DatasetCreateBody {
        // ... configure metadata
        ..Default::default()
    };
    let response = dataset::create_dataset(&client, "root", dataset_body).await?;
    
    // Upload a file
    let file = UploadFile::from("path/to/file.csv");
    let identifier = Identifier::PersistentId("doi:10.5072/FK2/ABCDEF".to_string());
    dataset::upload_file_to_dataset(&client, identifier, file, None, None).await?;
    
    Ok(())
}
```

**Key Library Modules:**

- `dataverse::client::BaseClient` - HTTP client for API interactions
- `dataverse::native_api::collection` - Collection operations
- `dataverse::native_api::dataset` - Dataset operations
- `dataverse::native_api::file` - File operations
- `dataverse::native_api::admin` - Administrative operations
- `dataverse::search_api` - Search functionality
- `dataverse::direct_upload` - Direct upload with parallel batch support
- `dataverse::data_access` - File and dataset downloads

### CLI Usage

The CLI provides three flexible authentication methods:

#### 1. Profile-Based (Recommended)

Store credentials securely in your system keyring:

```bash
# Create a profile
dvcli auth set --name production --url https://dataverse.org --token your-api-token

# Use the profile
dvcli --profile production info version
```

#### 2. Environment Variables

Set environment variables for automatic authentication:

```bash
export DVCLI_URL="https://demo.dataverse.org"
export DVCLI_TOKEN="your-api-token"

dvcli dataset meta doi:10.5072/FK2/ABC123
```

#### 3. Interactive Mode

If neither profile nor environment variables are set, the CLI will prompt for credentials:

```bash
dvcli info version
# Prompts for URL and token
```

**Common CLI Operations:**

> **Note:** Configuration files can be provided in both JSON and YAML formats.

```bash
# Get help
dvcli --help
dvcli dataset --help

# Collections
dvcli collection create --parent root --body collection.json
dvcli collection publish my-collection

# Datasets
dvcli dataset create --collection root --body dataset.json  # or dataset.yaml
dvcli dataset upload --id doi:10.5072/FK2/ABC123 data.csv
dvcli dataset publish doi:10.5072/FK2/ABC123

# Direct upload (faster for large files)
dvcli dataset direct-upload --id doi:10.5072/FK2/ABC123 --parallel 5 file1.csv file2.csv

# Files
dvcli file replace --id 12345 --path new-file.csv
dvcli file download file-pid.txt --path ./downloads/

# Search
dvcli search -q "climate change" -t dataset -t file

# Admin
dvcli admin storage-drivers
dvcli admin add-external-tool tool-manifest.json
```

## Examples

Complete workflow examples are available in the [`examples/`](examples/) directory:

- **[create-upload-publish](examples/create-upload-publish)** - End-to-end workflow demonstrating collection and dataset creation, file upload, and publishing using shell scripts and the CLI.

Besides these examples, you can also find some recipes in the [Dataverse Recipes](https://github.com/gdcc/dataverse-recipes/tree/main/dvcli) repository, which cover most of the functionality of the CLI.

## Development

### Running Tests

Tests require a running Dataverse instance. We provide a convenient test script that handles infrastructure setup:

```bash
# Run all tests (starts Docker containers automatically)
./run-tests.sh

# Run a specific test
./run-tests.sh test_create_dataset
```

The script automatically:

- Starts Dataverse with PostgreSQL and Solr via Docker Compose
- Waits for services to be ready
- Configures environment variables
- Executes the test suite

Docker containers remain running after tests complete for faster subsequent runs. View logs with `docker logs dataverse` if you encounter issues.

### Manual Test Setup

For granular control during development:

```bash
# Start infrastructure
docker compose -f ./docker/docker-compose-base.yml --env-file local-test.env up -d

# Configure environment
export BASE_URL=http://localhost:8080
export DV_VERSION=6.2
export $(grep "API_TOKEN" "dv/bootstrap.exposed.env")
export API_TOKEN_SUPERUSER=$API_TOKEN

# Run tests
cargo test
cargo test -- --nocapture  # with output
cargo test test_name       # specific test
cargo test collection::    # module tests
```

## Contributing

Contributions are welcome! Whether you're fixing bugs, adding features, or improving documentation, your help is appreciated. Please feel free to open issues or submit pull requests on [GitHub](https://github.com/JR-1991/rust-dataverse).

## Community

Join the conversation on the [Dataverse Zulip Channel](https://dataverse.zulipchat.com)! Connect with other developers, get help, share ideas, and discuss the future of Rust clients for Dataverse.

## License

This project is licensed under the MIT License - see the [License.md](License.md) file for details.
