#!/bin/bash
set -e -x

cd bindings/python

curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
rustup default 1.78.0

free -h
cargo build --jobs 1 --config net.git-fetch-with-cli=true

for PYBIN in /opt/python/cp3[891]*/bin; do
    "${PYBIN}/pip" install maturin
    "${PYBIN}/maturin" build -F python -i "${PYBIN}/python" --release
done

for wheel in target/wheels/*.whl; do
    auditwheel repair "${wheel}"
done