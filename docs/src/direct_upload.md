# Direct Upload

Direct upload is a feature of Dataverse that allows you to upload large files to Dataverse by directly uploading the file to a Dataverse-compatible storage backend. This is particularly useful for uploading files that exceed the size limitations of the Dataverse web interface. The primary advantage is the ability to upload large files in manageable chunks and to resume uploads in case of network failures.

## Technical Implementation

The direct upload process in Rust-Dataverse is implemented through several key components:

1. **Ticket Acquisition**: The process begins by obtaining an upload ticket from the Dataverse server (`get_ticket` in `upload.rs`).

2. **Upload Strategy Selection**: Based on the file size, the system determines whether to use single-part or multi-part upload:
   - Small files use a single HTTP request
   - Large files are split into multiple parts for parallel uploading

3. **Database Tracking**: For multi-part uploads, a SQLite database (`model.rs`) maintains:
   - Upload metadata (storage identifier, file path, size)
   - Individual part information (part number, byte range, upload status)
   - File hash information for integrity verification

4. **Resumable Uploads**: The system tracks which parts have been successfully uploaded, enabling resumption of interrupted uploads without restarting from the beginning.

5. **File Registration**: After all parts are uploaded, the file is registered with the dataset using the computed hash for integrity verification.

## Resumable Uploads

Unlike other implementations such as [python-dvuploader](https://github.com/gdcc/python-dvuploader) that upload files sequentially in runtime, Rust-Dataverse maintains a persistent local database to track upload progress. This database stores:

- Upload metadata (file path, size, storage identifier)
- Part information (byte ranges, URLs, upload status)
- File hash data for integrity verification

If an upload is interrupted for any reason (network failure, system crash, etc.), the system can query this database to determine which parts have already been uploaded and resume the process by uploading only the missing parts.

## Uploading a File or Directory

To upload a file or directory to Dataverse, use the `dvcli` command:

```bash
dvcli direct-upload \
    -i <ID> \
    <PATH>
```

Parameters:

- `id`: (Persistent) Identifier of the dataset.
- `path`: Path to the file or directory to upload.

This command will upload the specified files or directories to Dataverse, using the direct upload process. The response from the Dataverse server will be displayed in your terminal.

## Tab ingest

Sometimes tab-ingestion can lead to issues, when direct uploading high amounts of tabular data. In this case, it is recommended to disable tab-ingestion by setting the `tab-ingest` parameter to `false` in the Dataverse configuration file.

```bash
dvcli direct-upload <PATH> \
    -i <ID> \
    --tab-ingest false
```

This will disable tab-ingestion for the current session.

## Assessing remaining uploads

You can assess the remaining uploads by invoking the `dvcli direct-upload` command with the `remaining` subcommand:

```bash
dvcli direct-upload remaining
```

This will display a list of all the remaining uploads, together with their progress. If you wish to remain an upload, you can do so by invoking the `dvcli direct-upload remain <upload-id>` command:

```bash
dvcli direct-upload remain <upload-id>
```
