#!/bin/bash
set -e -x

# check if we running in our dev Docker or directory on the build machine
# by checking if the current directory ends with extract-rs/bindings/python
current_dir=$(pwd)


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


if [[ $current_dir =~ extract-rs/bindings/python$ ]]; then
    echo "Build launched not on Docker but on developer machine ..."
    pip install -q maturin
    maturin build


elif [[ $current_dir =~ ^/workdir ]]; then
    echo "Build launched from inside docker ..."
    cd /workspace/bindings/python

    for PYBIN in /opt/python/cp38*/bin; do
        "${PYBIN}/pip" install maturin
        "${PYBIN}/pip" install wheel
        #"${PYBIN}/maturin" build -i "${PYBIN}/python" --release --out /workspace/bindings/python/dist --compatibility manylinux_2_34
        "${PYBIN}/maturin" build -i "${PYBIN}/python" --out /workspace/bindings/python/dist --compatibility manylinux_2_28
    done

else
    echo "Please make sure to run the script from extract-rs/bindings/python or inside docker"
    exit 1
fi


