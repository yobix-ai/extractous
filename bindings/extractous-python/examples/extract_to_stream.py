#!/usr/bin/env python3
import os
import sys

from extractous import Extractor, PdfOcrStrategy, PdfParserConfig


def extract_to_stream(file_path: str):

    # Extract the file
    extractor = Extractor()
    reader = extractor.extract_file(in_file)

    buffer = bytearray(4096 * 4096)
    while True:
        bytes_read = reader.readinto(buffer)
        # If no more data, exit the loop
        if bytes_read == 0:
            break
        # Decode the valid portion of the buffer and append it to the result
        chunk = buffer[:bytes_read].decode('utf-8')
        print(chunk)


if __name__ == '__main__':
    # Pare input args
    if len(sys.argv) != 2:
        print(f"Usage: '{sys.argv[0]}' <filename>")
        sys.exit(1)
    in_file = sys.argv[1]
    if not os.path.isfile(in_file):
        raise FileNotFoundError(f"No such file: '{in_file}'")

    extract_to_stream(in_file)
