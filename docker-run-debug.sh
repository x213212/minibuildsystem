#!/bin/bash

# Ensure at least two arguments are provided
if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <image_tag> <script_name> [args...]"
    exit 1
fi

# Get Docker image tag and the name of the script to execute
TAG=$1
SCRIPT_NAME=$2
shift 2
ARGS="$@"

# Ensure the tag is not empty
if [ -z "$TAG" ]; then
    echo "Error: Docker image tag must not be empty."
    exit 1
fi

# Use the absolute path to mount the current directory to /app
CURRENT_DIR=$(pwd)

# Clean up old build artifacts
cargo clean

# Run the Docker container and execute cargo run
docker run --rm -it -v "$CURRENT_DIR:/app" "$TAG" bash -c "cargo run --manifest-path /app/Cargo.toml $SCRIPT_NAME $ARGS"
