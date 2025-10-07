#!/bin/bash

# Check if Docker is installed
if ! command -v docker &>/dev/null; then
    echo "✋ Docker is not installed. Please install Docker before running this script."
    exit 1
fi

# Parse command line arguments
TEST_NAME=""
if [ $# -gt 0 ]; then
    TEST_NAME="$1"
fi

# Prepare the environment for the test
mkdir dv >>/dev/null 2>&1
touch dv/bootstrap.exposed.env >>/dev/null 2>&1

printf "\n🚀 Preparing containers\n"

# Start all containers (infrastructure + tests)
printf "\n🚀 Starting all containers...\n"
printf "   The test container will wait for Dataverse and fetch the version automatically\n\n"

docker compose \
    -f docker/docker-compose-base.yml \
    --env-file local-test.env \
    up -d

printf "\n🚀 Waiting for Dataverse to be ready...\n"

# Wait for Dataverse to be ready by checking the version endpoint
max_attempts=60
attempt=1
while [ $attempt -le $max_attempts ]; do
    printf "   Attempt $attempt/$max_attempts: Checking if Dataverse is ready...\n"
    
    if curl -s -f http://localhost:8080/api/info/version > /dev/null 2>&1; then
        printf "   ✅ Dataverse is ready!\n"
        break
    fi
    
    if [ $attempt -eq $max_attempts ]; then
        printf "   ❌ Dataverse failed to start after $max_attempts attempts\n"
        printf "   📋 Checking container logs...\n"
        docker logs dataverse
        exit 1
    fi
    
    printf "   ⏳ Dataverse not ready yet, waiting 10 seconds...\n"
    sleep 10
    attempt=$((attempt + 1))
done

# Fetch the env variables from the container
printf "\n🚀 Fetching environment variables from container...\n"

# Copy the bootstrap.exposed.env file from the container to get the API_TOKEN
docker cp dataverse:/dv/bootstrap.exposed.env dv/bootstrap.exposed.env

# Source the environment variables
if [ -f dv/bootstrap.exposed.env ]; then
    export $(grep "API_TOKEN" "dv/bootstrap.exposed.env")
    export API_TOKEN_SUPERUSER=$API_TOKEN
    export BASE_URL=http://localhost:8080
    export DV_VERSION=$(curl -s -f http://localhost:8080/api/info/version | jq -r '.data.version')
    printf "   ✅ Environment variables loaded successfully\n"
    printf "   📋 API_TOKEN: ********\n"
    printf "   📋 BASE_URL: ${BASE_URL}\n"
    printf "   📋 DV_VERSION: ${DV_VERSION}\n"
else
    printf "   ❌ Failed to fetch bootstrap.exposed.env from container\n"
    exit 1
fi

# When ready, run cargo test
if [ -n "$TEST_NAME" ]; then
    printf "\n🚀 Running specific test: $TEST_NAME...\n"
    cargo test "$TEST_NAME"
else
    printf "\n🚀 Running all tests...\n"
    cargo test
fi
