# Datasets

Datasets are the central objects in Dataverse and contain files and other metadata. These structured repositories hold your research data, documentation, and associated metadata, providing a comprehensive package that can be versioned, cited, and shared according to your specifications. DVCLI provides comprehensive commands to efficiently manage datasets throughout their lifecycle.

## Creation

Create new datasets using the `dvcli dataset create` command. This operation establishes a new dataset container in your specified collection, ready to be populated with files and metadata according to your research or data sharing requirements.

```bash
dvcli dataset create \
    --body <PATH> \
    --collection <ID>
```

Parameters:

- `body`: Path to the JSON/YAML file containing the dataset body.
- `collection`: Alias of the collection to create the dataset in.

This command creates a dataset in the specified collection and returns the dataset metadata from the Dataverse server. If the collection does not exist, the command will return an appropriate error.

> **Note**: To save the server response for future use, add the `>>` directive followed by a filepath:
>
> ```bash
> dvcli dataset create --body <PATH> --collection <ID> >> <PATH>
> ```
>
> DVCLI will strip the progress indicators and formatting elements, providing only the raw response.

## Deletion

Remove datasets using the `dvcli dataset delete` command. This operation permanently removes an unpublished dataset from the Dataverse repository, which is useful during development or when data needs to be completely withdrawn.

```bash
dvcli dataset delete <ID>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset to delete.

This command handles the deletion process and returns the metadata of the deleted dataset.

## Editing

Modify dataset metadata with the `dvcli dataset edit` command. This functionality allows you to update metadata fields, add new metadata, or completely replace the existing metadata structure, ensuring your dataset information remains accurate and up-to-date.

```bash
dvcli dataset edit \
    --body <PATH> \
    --replace <BOOLEAN> \
    <ID>
```

Parameters:

- `body`: Path to the JSON/YAML file containing the edits to the dataset.
- `replace`: Whether to replace the entire dataset metadata (`true`) or merge the edits with existing metadata (`false`).
- `id`: (Persistent) Identifier of the dataset to edit.

> **Important**: This command follows the Dataverse API's [`editMetadata` endpoint](https://guides.dataverse.org/en/latest/api/native-api.html#edit-dataset-metadata). The default setting (`--replace false`) merges edits with existing metadata but may not support all edit operations. For compound collections, using `--replace true` is often necessary for complete replacement.

## Fetching Metadata

Retrieve dataset metadata using the `dvcli dataset metadata` command. This operation provides access to the complete metadata package of a dataset, allowing you to inspect, verify, or extract specific information for reporting or further processing.

```bash
dvcli dataset metadata \
    --version <VERSION> \
    <ID>
```

Parameters:

- `version`: Version of the dataset to fetch. Defaults to `:latest` and `:draft` when an API key is provided.
- `id`: (Persistent) Identifier of the dataset to fetch.

This command retrieves comprehensive metadata that can be used for inspection or as a template for future operations. For specific file metadata, the `dvcli dataset list-files` command offers a more targeted approach.

## Publishing

Publish datasets using the `dvcli dataset publish` command. Publishing finalizes a dataset version, making it citable and publicly available according to your access controls, and creates a permanent record of the dataset at that specific version state.

```bash
dvcli dataset publish \
    --version <VERSION> \
    <ID>
```

Parameters:

- `version`: Version type to publish (major, minor, updatecurrent) [default: major]
- `id`: (Persistent) Identifier of the dataset to publish.

> **Version types explained**:
>
> - `major`: Increments the major version (e.g., 1.0 → 2.0)
> - `minor`: Increments the minor version (e.g., 1.0 → 1.1)
> - `updatecurrent`: Updates the current version without incrementing the version number

## Uploading Files

DVCLI provides multiple approaches for adding files to datasets. The file upload functionality allows you to populate your dataset with actual data files, supporting documentation, and supplementary materials in various formats and organizational structures.

### Uploading a Single File

```bash
dvcli dataset upload \
    --body <PATH> \
    --id <ID> \
    --dv-path <PATH> \
    <PATH>
```

Parameters:

- `body`: Path to the JSON/YAML file containing the file metadata.
- `id`: (Persistent) Identifier of the dataset.
- `dv-path`: The `directoryLabel` path within the dataset (e.g., `data` for a data directory). This is not the path on your local system.
- `path`: Path to the file on your local system.

### Uploading Directories

Upload entire directories efficiently. This approach maintains your directory structure and allows for logical organization of related files within your dataset.

```bash
dvcli dataset upload \
    --body <PATH> \
    --id <ID> \
    --dv-path <PATH> \
    <PATH>
```

This creates a zip stream of the directory and handles extraction on the server side, providing a more efficient alternative to individual file uploads.

### Direct Upload

For advanced scenarios involving large files or multiple simultaneous uploads, DVCLI provides a direct upload feature. See the [Direct Upload section](./direct-upload.md) for detailed information.

## Exporting

Export datasets in standardized formats. This capability enables interoperability with other systems and fulfills requirements for data sharing according to community standards and best practices.

```bash
dvcli dataset export \
    --id <ID> \
    --format <FORMAT> \
    --out <PATH>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset to export.
- `format`: Format for export (e.g., 'ddi', 'oai_ddi', 'datacite').
- `out`: Path where the exported file will be saved.

This facilitates compatibility with standards such as [MLCommons Croissant](https://mlcommons.org/working-groups/data/croissant/) and [DataCite](https://datacite.org).

## Linking to Collections

Connect datasets to collections using the `dvcli dataset link` command. This functionality allows you to organize related datasets within thematic collections, improving discoverability and providing contextual relationships between datasets.

```bash
dvcli dataset link \
    --id <ID> \
    --collection <ID>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset to link.
- `collection`: (Persistent) Identifier of the collection to link the dataset to.

This command establishes the relationship between the specified dataset and collection, returning the updated dataset metadata.

## Locks

Dataset locks provide protection against unintended modifications. Locks are an important safeguard mechanism that prevent simultaneous edits, protect datasets during critical operations, and maintain data integrity throughout processing workflows.

### Getting the Lock Status

```bash
dvcli dataset lock <ID>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset.

### Getting Lock Status by Type

```bash
dvcli dataset lock --type <TYPE> <ID>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset.
- `type`: The specific lock type to check.

> **Available lock types**:
>
> - `ingest`
> - `workflow`
> - `in-review`
> - `finalize-publication`
> - `edit-in-progress`
> - `file-validation-failed`

### Setting a Lock

```bash
dvcli dataset set-lock <ID> \
    --type <TYPE> \
    --set
```

Parameters:

- `id`: (Persistent) Identifier of the dataset.
- `type`: The type of lock to apply.
- `set`: Flag to enable the lock.

### Removing a Lock

```bash
dvcli dataset locks <ID> \
    --remove \
    --type <TYPE>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset.
- `remove`: Flag to remove the lock.
- `type`: The type of lock to remove.

## Submitting for Review

Submit datasets for review using the `dvcli dataset submit-for-review` command. This initiates a formal review process where dataset curators or administrators can verify the completeness, accuracy, and compliance of your dataset before it proceeds to publication.

```bash
dvcli dataset submit-for-review <ID>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset to submit.

> **Note**: This operation automatically applies a lock on the dataset, preventing edits until the review is complete. Use `dvcli dataset lock` to check the current lock status.

## File Download

Download individual files from a dataset. This targeted download capability allows you to retrieve specific files of interest without downloading the entire dataset, saving bandwidth and storage space.

```bash
dvcli dataset download <ID> \
    --out <DIR>
```

Parameters:

- `id`: (Persistent) Identifier of the file to download.
- `out`: Target directory for the downloaded file (defaults to current working directory).

## Dataset Download

Download complete datasets. This comprehensive download option retrieves all files and metadata associated with a dataset, providing a complete local copy for analysis, archiving, or redistribution.

```bash
dvcli dataset download <ID> \
    --complete \
    --out <DIR>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset to download.
- `complete`: Flag to download the entire dataset.
- `out`: Target directory for the downloaded content.
