from extractous import Extractor
from utils import read_to_string


def expected_result():
    return "\nHello Quarkus\n\n\n"


def test_extract_file_to_string():
    extractor = Extractor()
    result, metadata = extractor.extract_file_to_string("tests/quarkus.pdf")

    #print(result)
    assert result == expected_result()

def test_extract_file():
    extractor = Extractor()
    reader, metadata = extractor.extract_file("tests/quarkus.pdf")

    result = read_to_string(reader)

    #print(result)
    assert result == expected_result()

def test_extract_bytes():
    extractor = Extractor()

    with open("tests/quarkus.pdf", "rb") as file:
        buffer = bytearray(file.read())
    reader, metadata = extractor.extract_bytes(buffer)

    result = read_to_string(reader)

    #print(result)
    assert result == expected_result()
