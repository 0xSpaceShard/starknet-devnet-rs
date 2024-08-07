#!/bin/bash
set -euo pipefail

# Cache is available only on our custom runner
if [ "$CIRCLECI_RUNNER_CLASS" != "spaceshard/ax41" ]; then
    echo "Custom cache available only on self-hosted spaceshard/ax41 runner"
    exit 0
fi

if [ $# -ne 1 ]; then
    echo "Please provide cache action as an argument. (save, load or cleanup)"
    exit 1
fi

action="$1"

# VARIABLES
# Files to use as cache key
cache_key_files=("Cargo.lock" "rust-toolchain.toml") 

 # Directories to cache
cached_dirs=("target/release/.fingerprint target/release/build target/release/deps")

# Cache files that are accessed more than $cache_cleanup_interval days ago will be removed in cleanup step
cache_cleanup_interval=7 


cache_base_dir="/cache" # dependent on runner architecture
cache_file="$cache_base_dir/$(cat ${cache_key_files[@]} | shasum | awk '{print $1}').tar.gz"

case "$action" in
    "load")
        echo "Loading cache..."
        if [ ! -f "$cache_file" ]; then
            echo "Cache does not exist."
            exit 0
        fi
        tar xvf "$cache_file"
        ;;
    "save")
        echo "Creating cache..."
        if [ -f "$cache_file" ]; then
            echo "Cache already exists."
            exit 0
        fi
        tmpfile="$(mktemp -p $cache_base_dir)"
        tar czvf "$tmpfile" ${cached_dirs[@]} # This is to handle race condition. One runner might be loading cache while another isn't done saving it. Or multple runners might be saving cache at the same time.
        mv "$tmpfile" "$cache_file"
        ;;
    "cleanup")
        echo "Cleaning up cache..."
        find "$cache_base_dir" -amin +30 -not -name '*.gz' -exec rm {} \; # Remove temporary cache files if they are leftover (older than 30 minutes, for race condition)
        find "$cache_base_dir" -atime "+$cache_cleanup_interval" -name '*.gz' -exec rm {} \; # Remove nonactive cache files
        exit 0
        ;;
    *)
        echo "Invalid action $action. Valid actions are save, load, and cleanup".
        exit 1
        ;;
esac
