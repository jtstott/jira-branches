#!/bin/bash
initial_wd=$(pwd)
cd "$(dirname "$0")" || exit

if ! command -v rustup &> /dev/null
then
    echo "Installing Rust..."
    curl https://sh.rustup.rs -sSf | sh
fi

echo "Building binary for your system..."
cargo build --release

cp target/release/jira_branches "$initial_wd/jb"

pwd
echo "$initial_wd"