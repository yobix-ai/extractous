#!/bin/bash

if [[ $PWD =~ extract-rs/bindings/python$ ]]; then
    ROOT_DIR=$(realpath "$PWD/../../")
    docker build $PWD
    docker run --rm --mount type=bind,source=$ROOT_DIR,target=/workspace manylinux_2_28_graalvm bash /workspace/bindings/python/build-wheels.sh

    # reset paemissions
    echo ""
    echo "Resettings permissions on some directories that were touched by docker running in root "
    sudo chown -R $USER:$USER $ROOT_DIR
else
    echo "Please make sure to run the script from extract-rs/bindings/python"
    exit 1
fi


