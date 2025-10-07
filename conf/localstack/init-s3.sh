#!/bin/bash

# Create the mybucket bucket for Dataverse S3 storage
awslocal s3 mb s3://mybucket

echo "S3 bucket 'mybucket' created successfully"

