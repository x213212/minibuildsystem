#!/bin/bash

# Ensure the Dockerfile exists in the current directory
DOCKERFILE_PATH="./dockerfile/rockylinux.dockerfile"
if [ ! -f "$DOCKERFILE_PATH" ]; then
    echo "Error: Dockerfile not found at $DOCKERFILE_PATH."
    exit 1
fi

# Generate a timestamp as the tag
TAG="rsbuild_image:$(date +%Y%m%d%H%M%S)"

# Build the Docker image
if ! docker build -t "$TAG" -f "$DOCKERFILE_PATH" .; then
    echo "Error: Failed to build Docker image."
    exit 1
fi

# Output the generated tag
echo "Docker image built successfully with tag: $TAG"
