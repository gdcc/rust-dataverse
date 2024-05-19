# Get the collection alias for later
alias=$(jq -r '.alias' collection.json)

# Create both a collection and a dataset
dvcli collection create --parent Root --body collection.json
dvcli dataset create --collection $alias --body dataset.json >> dataset_output.json

# Get the persistent ID of the dataset and upload a file
persistent_id=$(jq -r '.persistentId' dataset_output.json)
dvcli file upload files/data.csv \
    --id $persistent_id \
    --body file.json

# Publish the dataset and collection
dvcli collection publish $alias
dvcli dataset publish $persistent_id
