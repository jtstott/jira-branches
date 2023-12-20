#!/bin/bash

# Set working directory
cd "$(dirname "$0")" || exit

# Get current system target
TARGET=$(rustc -vV | sed -n 's|host: ||p')

# Build release binary
cargo build -r

# Copy binary output to bin/
mkdir -p "./bin/"
cp ./target/release/jb ./bin/"jb-$TARGET"
