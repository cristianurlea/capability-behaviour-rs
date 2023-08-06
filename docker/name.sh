#!/bin/bash

# Define the image name you are looking for
TARGET_IMAGE="nas.local:8443/rust.img"

# Find the container ID of the running container with the specified image
CONTAINER_ID=$(docker ps --filter "status=running" --filter "ancestor=${TARGET_IMAGE}" --format "{{.ID}}")

# Check if the container ID is empty (no running container found with the specified image)
if [ -z "$CONTAINER_ID" ]; then
  echo "No running container found with image: $TARGET_IMAGE"
  exit 1
fi

# Get the name of the running container based on the container ID
CONTAINER_NAME=$(docker inspect --format '{{.Name}}' "$CONTAINER_ID" | sed 's/^\///')

# Pass the container name as an environment variable to your other script
export INSTANCE_NAME="$CONTAINER_NAME"

# Call your other script here, and it can access the container name using the environment variable $INSTANCE_NAME
# For example:
# ./path/to/your_other_script.sh

echo "Instance name of the running container: $INSTANCE_NAME"
CONTAINER_NAME="$INSTANCE_NAME" ./copy.sh

