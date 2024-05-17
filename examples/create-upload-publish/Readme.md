# Workflow #1

This workflow example demonstrates how to use DVCLI to perform the following operations:

* Create a new collection in Dataverse instance
* Create a new dataset in the collection
* Upload a file to the dataset
* Publish the collection and dataset

## Prerequisites

For these commands to work, you need to supply the following environment variables:

* `DVCLI_URL` - The URL of the Dataverse instance
* `DVCLI_TOKEN` - The API token for the Dataverse instance

```bash
export DVCLI_URL="http://localhost:8080"
export DVCLI_TOKEN="<API_TOKEN>"
```

### Files

* `collection.json` - Metadata file to create a collection
* `dataset.json` - Metadata file to create a dataset
* `data.csv` - Sample data file
* `file.json` - Metadata file to create a file

## Steps

### 1. Create a new collection in Dataverse instance

This command creates a new collection in the Dataverse instance with the metadata provided in the `collection.json` file. As demonstrated below, you can also save the JSON output to a file for further processing. This will only save the JSON response and strip all the other fancy messages.

```bash
# Print the JSON output to the terminal
dvcli collection create --parent Root --body collection.json

# You can also save the JSON output
# to a file (which is useful for further processing)
dvcli collection create --parent Root --body collection.json >> collection_output.json
```

```bash
🎉 Success! - Received the following response:

{
  "affiliation": "University of Dataverse",
  "alias": "dvcli",
  "creationDate": "2024-05-17T15:38:28Z",
  "dataverseContacts": [
    {
      "contactEmail": "john@doe.com",
      "displayOrder": 0
    }
  ],
  "dataverseType": "TEACHING_COURSES",
  "description": "This dataverse was created by DVCLI",
  "id": 318,
  "isReleased": false,
  "ownerId": 1,
  "permissionRoot": true
}
```

### 2. Create a new dataset in the collection

This command creates a new dataset in the collection `dvcli` with the metadata provided in the `dataset.json` file.

```bash
dvcli dataset create --collection dvcli --body dataset.json >> dataset_output.json
```

```bash
🎉 Success! - Received the following response:

{
  "id": 319,
  "persistentId": "doi:10.5072/FK2/YNRRF6"
}
```

### 3. Upload a file to the dataset

This command uploads the file `data.csv` to the dataset created in the previous step. Keep in mind, that re-running this command will not overwrite the existing file, but will attach a `-[digit]` to the filename.

```bash
# Use jq to extract the persistent ID from the JSON output dataset_output.json
# and save it to a variable
persistent_id=$(jq -r '.persistentId' dataset_output.json)

# Upload the file to the dataset
dvcli file upload files/data.csv --pid $persistent_id --body file.json
```

```bash
🎉 Success! - Received the following response:

{
  "files": [
    {
      "categories": [
        "Data"
      ],
      "datasetVersionId": 79,
      "description": "My description.",
      "label": "data.csv",
      "restricted": false,
      "version": 1
    }
  ]
}
```

### 4. Publish the collection and dataset

These commands publish the collection and dataset, making them publicly accessible.

```bash
dvcli collection publish dvcli
dvcli dataset publish --version major $persistent_id
```

```bash
🎉 Success! - Received the following response:

{
  "affiliation": "University of Dataverse",
  "alias": "dvcli",
  "creationDate": "2024-05-17T15:38:28Z",
  "dataverseContacts": [
    {
      "contactEmail": "john@doe.com",
      "displayOrder": 0
    }
  ],
  "dataverseType": "TEACHING_COURSES",
  "description": "This dataverse was created by DVCLI",
  "id": 318,
  "isReleased": true,
  "ownerId": 1,
  "permissionRoot": true
}

🎉 Success! - Received the following response:

{
  "authority": "10.5072",
  "id": 319,
  "identifier": "FK2/YNRRF6",
  "persistentUrl": "https://doi.org/10.5072/FK2/YNRRF6",
  "protocol": "doi",
  "publisher": "Root",
  "storageIdentifier": "local://10.5072/FK2/YNRRF6"
}
```
