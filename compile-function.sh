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
cargo build > /dev/termination-log 2>&1
mv ./target/debug/server $KUBELESS_INSTALL_VOLUME
mv ./Rocket.toml $KUBELESS_INSTALL_VOLUME
