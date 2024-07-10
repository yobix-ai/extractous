#!/bin/bash

# Check if the correct number of arguments are provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <wheel_dir>"
    exit 1
fi

WHEEL_DIR=$1

# Check if the provided wheel directory exists
if [ ! -d "$WHEEL_DIR" ]; then
    echo "Wheel directory does not exist: $WHEEL_DIR"
    exit 1
fi

# Ensure wheel and patchelf are installed
if ! command -v wheel &> /dev/null
then
    echo "wheel could not be found, please install it with pip install wheel"
    exit 1
fi

if ! command -v patchelf &> /dev/null
then
    echo "patchelf could not be found, please install it"
    exit 1
fi

# Find all wheel files in the specified directory
WHEEL_FILES=$(find "$WHEEL_DIR" -name "*.whl")

# Check if any wheel files were found
if [ -z "$WHEEL_FILES" ]; then
    echo "No wheel files found in the directory: $WHEEL_DIR"
    exit 1
fi

# Loop through each wheel file and perform the required operations
for WHEEL_FILE in $WHEEL_FILES; do
    echo "Processing $WHEEL_FILE ..."

    # Unpack the wheel file into the wheel directory
    python -m wheel unpack "$WHEEL_FILE" -d "$WHEEL_DIR"

    # Find the directory containing the unpacked wheel contents
    UNPACKED_WHEEL_DIR=$(find "$WHEEL_DIR" -mindepth 1 -maxdepth 1 -type d -name "extract_rs*")

    # Find the .so file in the extractrs directory
    SO_FILE=$(find "$UNPACKED_WHEEL_DIR" -name "_extractrs*.so" | head -n 1)

    # Check if the .so file exists
    if [ -z "$SO_FILE" ]; then
        echo "No file starting with _extractrs found in the extractrs directory of $WHEEL_FILE"
        continue
    fi

    # Patch the .so file to set its rpath to $ORIGIN/libs
    patchelf --set-rpath '$ORIGIN/libs' "$SO_FILE"

    # install_name_tool -add_rpath "@loader_path/libs" "$SO_FILE"

    # Pack the wheel again
    python -m wheel pack "$UNPACKED_WHEEL_DIR" -d "$WHEEL_DIR"

    # Clean up the unpacked directory
    rm -rf "$UNPACKED_WHEEL_DIR"

    echo "Wheel file $WHEEL_FILE has been patched and repacked successfully."
done
