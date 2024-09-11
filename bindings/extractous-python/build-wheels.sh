#!/bin/bash

# check if we running in our dev Docker or directory on the build machine
# by checking if the current directory ends with extractous/bindings/extractous-python
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


if [[ $current_dir =~ extractous/bindings/extractous-python$ ]]; then
    echo "maturin build launched not on Docker but on developer machine ..."
    echo ""

    pip install -q maturin
    maturin build --release
    # This tags the wheel with manylinux_2_34 check your version of glibc by running: ldd --version
    #maturin build --release --compatibility manylinux_2_34


elif [[ $current_dir =~ ^/workspace ]]; then
    echo "maturin build launched from inside docker ..."
    echo ""

    cd /workspace/bindings/extractous-python

    for PYBIN in /opt/python/cp38*/bin; do
        "${PYBIN}/pip" install maturin
        "${PYBIN}/pip" install wheel
        #"${PYBIN}/maturin" build -i "${PYBIN}/python" --release --out /workspace/bindings/extractous-python/dist --compatibility manylinux_2_34
        "${PYBIN}/maturin" build --release -i "${PYBIN}/python" --out /workspace/bindings/extractous-python/target/wheels --compatibility manylinux_2_28
    done

else
    echo "Please make sure to run the script from extractous/bindings/extractous-python or inside docker"
    exit 1
fi