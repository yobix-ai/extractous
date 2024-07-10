#!/bin/bash

# Check if the correct number of arguments are provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <wheel_file>"
    exit 1
fi

WHEEL_FILE=$1

# Check if the provided wheel file exists
if [ ! -f "$WHEEL_FILE" ]; then
    echo "Wheel file does not exist: $WHEEL_FILE"
    exit 1
fi

# Deduce wheel_dir from the full path of the wheel file
WHEEL_DIR=$(dirname "$WHEEL_FILE")

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

# Unpack the wheel file into the wheel directory
python -m wheel unpack "$WHEEL_FILE" -d "$WHEEL_DIR"

# Find the directory containing the unpacked wheel contents
UNPACKED_WHEEL_DIR=$(find "$WHEEL_DIR" -mindepth 1 -maxdepth 1 -type d -name "extract_rs*")

# Find the .so file that starts with _extractrs
SO_FILE=$(find "$UNPACKED_WHEEL_DIR" -name "_extractrs*.so" | head -n 1)

# Check if the .so file exists
if [ ! -f "$SO_FILE" ]; then
    echo "No file starting with _extractrs found in the wheel"
    exit 1
fi

# Patch the .so file to set its rpath to $ORIGIN/libs
patchelf --set-rpath '$ORIGIN/libs' "$SO_FILE"

# Pack the wheel again
python -m wheel pack "$UNPACKED_WHEEL_DIR" -d "$WHEEL_DIR"

# Clean up the unpacked directory
rm -rf "$UNPACKED_WHEEL_DIR"

echo "Wheel file has been patched and repacked successfully."
