#!/bin/bash
set -e -x

echo "building artifacts"
cd "${SRC_DIR}"

exec cargo build $RELEASE_FLAG $CRATES
