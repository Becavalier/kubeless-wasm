#!/bin/bash
set -e

# Build the function.
# We do this before the initialization of the init containers, rather than here.
# cd /server
# cargo install --path .
# cargo build > /dev/termination-log 2>&1
mv /server $KUBELESS_INSTALL_VOLUME
