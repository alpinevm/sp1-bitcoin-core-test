#!/usr/bin/env bash
set -e

function usage() {
    echo
    echo "Usage: $(basename "$0") [OPTIONS] bitcoin-core-version"
    echo
    echo "OPTIONS:"
    echo
    echo " -f    Vendor even if there are local changes to the rust-bitcoinconsensus git index"
    echo " -h    Print this help and exit"
    echo
    echo "Example:"
    echo
    echo "    $(basename "$0") -f v0.21.2"
    echo

    exit 0
}

# Check if the user asked for help or didn't provide a version argument
if (($# < 1)) || [ "$1" == '-h' ]; then
   usage
fi

# Set default variables

if [ -z "$CORE_VENDOR_GIT_ROOT" ]; then
    CORE_VENDOR_GIT_ROOT="$(git rev-parse --show-toplevel)"
else
    CORE_VENDOR_GIT_ROOT="$(realpath "$CORE_VENDOR_GIT_ROOT")"
fi

DEFAULT_DEPEND_DIR="src/native/vendor"  # Specify the correct base directory here
DEFAULT_CORE_REPO="https://github.com/bitcoin/bitcoin.git"

# Set up directory and repo variables with fallback to defaults
CORE_VENDOR_DEPEND_DIR="${CORE_VENDOR_DEPEND_DIR:-$DEFAULT_DEPEND_DIR}"
CORE_VENDOR_REPO="${CORE_VENDOR_REPO:-$DEFAULT_CORE_REPO}"

# Avoid duplicating paths in the target directory
DIR="$CORE_VENDOR_DEPEND_DIR/bitcoin"

# Command-line option parsing
CORE_REV=""
FORCE=no
while (( "$#" )); do
    case "$1" in
    -h)
        usage
        ;;
    -f)
        FORCE=yes
        ;;
    *)
        if [ -z "$CORE_REV" ]; then
            CORE_REV="$1"
        else
            echo "WARNING: ignoring unknown command-line argument $1"
        fi
        ;;
    esac
    shift
done

# Ensure the version is specified
if [ -z "$CORE_REV" ]; then
    echo "ERROR: You must specify a Bitcoin Core version to vendor."
    usage
fi

echo "Vendoring Bitcoin Core version: $CORE_REV to $DIR"
echo

# Check for uncommitted changes
if [ "$FORCE" == "no" ]; then
    if ! git diff --quiet -- "*.rs"; then
        echo "ERROR: There appear to be modified source files. Check these in or pass -f."
        exit 2
    fi
    if ! git diff --quiet -- "$CORE_VENDOR_DEPEND_DIR"; then
        echo "ERROR: The depend directory appears to be modified. Check it in or pass -f."
        exit 2
    fi
fi

# Create and clean the target directory
mkdir -p "$CORE_VENDOR_DEPEND_DIR"
rm -rf "$DIR" || true

# Clone the repo or copy from a local directory
if [ "$CORE_VENDOR_CP_NOT_CLONE" == "yes" ]; then
    cp -r "$CORE_VENDOR_REPO" "$DIR"
    chmod -R +w "$DIR"
else
    git clone "$CORE_VENDOR_REPO" "$DIR"
fi

# Check out the specified version
cd "$DIR"
git checkout "$CORE_REV"
SOURCE_REV=$(git rev-parse HEAD || echo "[unknown revision from $CORE_VENDOR_REPO]")

# Inject configuration file into the correct src/config directory of Bitcoin Core
mkdir -p "src/config"
cat > "src/config/bitcoin-config.h" << 'EOL'
#ifndef BITCOIN_CONFIG_H
#define BITCOIN_CONFIG_H

// Configuration settings for Bitcoin Core (customized for vendoring)

#endif // BITCOIN_CONFIG_H
EOL

# Remove .git directory for vendoring
rm -rf .git/ || true

# Record revision information
echo "# This file was automatically created by $(basename "$0")" > "$CORE_VENDOR_DEPEND_DIR/bitcoin-HEAD-revision.txt"
echo "$SOURCE_REV" >> "$CORE_VENDOR_DEPEND_DIR/bitcoin-HEAD-revision.txt"

echo "Bitcoin Core vendoring completed successfully in $CORE_VENDOR_DEPEND_DIR."
