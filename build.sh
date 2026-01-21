#!/bin/bash
set -e

# Build mediaremote-adapter
cd mediaremote-adapter
mkdir -p build
cd build
cmake ..
make
cd ../..

# Prepare assets
mkdir -p assets

# Create tarball
# Copy to a temp location to ensure clean tar structure
TMP_DIR=$(mktemp -d)
cp mediaremote-adapter/bin/mediaremote-adapter.pl "$TMP_DIR/"
cp -R mediaremote-adapter/build/MediaRemoteAdapter.framework "$TMP_DIR/"

tar -czf assets/mediaremote-adapter.tar.gz -C "$TMP_DIR" mediaremote-adapter.pl MediaRemoteAdapter.framework

rm -rf "$TMP_DIR"

echo "Build and compression complete: assets/mediaremote-adapter.tar.gz"
