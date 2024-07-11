#!/bin/bash
set -e -x

# # install graalvm
# yum install -y zip
# curl -s "https://get.sdkman.io" | sh -s -- -y
# source "$HOME/.sdkman/bin/sdkman-init.sh"

# sdk install java 22.0.1-graalce
# sdk use java 22.0.1-graalce
# echo "JAVA_HOME: $JAVA_HOME"

# # install rust
# curl https://sh.rustup.rs -sSf | sh -s -- -y
# source $HOME/.cargo/env
# rustup default 1.78.0

cd /workspace/bindings/python

free -h
#cargo build --jobs 1 --config net.git-fetch-with-cli=true

for PYBIN in /opt/python/cp310*/bin; do
    "${PYBIN}/pip" install maturin
    "${PYBIN}/pip" install wheel
    "${PYBIN}/maturin" build -i "${PYBIN}/python" --release --out /workspace/bindings/python/dist --compatibility manylinux_2_34
done
