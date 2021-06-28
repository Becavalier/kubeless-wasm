#!/bin/bash

set -e

# Copy function
cp -r /kubeless/* /server/function/

# Replace FUNCTION placeholder
# sed "s/<<FUNCTION>>/${KUBELESS_FUNC_NAME}/g" /server/kubeless.go.tpl > /server/kubeless.go

# Build command
cd /server

# Build the function and redirect stdout & stderr from the compilation step to the k8s output log
cargo install --path .
cargo build --release -Z unstable-options --out-dir $KUBELESS_INSTALL_VOLUME > /dev/termination-log 2>&1
