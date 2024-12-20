from extractous import Extractor
from utils import read_to_string, extract_body_text


def expected_result():
    return "\nHello Quarkus\n\n\n"


def test_extract_file_to_string():
    extractor = Extractor()
    result, metadata = extractor.extract_file_to_string("tests/quarkus.pdf")

    print(f"test_pdf:test_extract_file_to_string result = {result}")
    assert result == expected_result()

def test_extract_file():
    extractor = Extractor()
    reader, metadata = extractor.extract_file("tests/quarkus.pdf")

    result = read_to_string(reader)

    print(f"test_pdf:test_extract_file result = {result}")
    assert result == expected_result()

def test_extract_file_as_xml():
    extractor = Extractor()
    extractor = extractor.set_xml_output(True)
    reader, metadata = extractor.extract_file("tests/quarkus.pdf")

    result_xml = read_to_string(reader)

    print(f"test_pdf:test_extract_file_as_xml result = {result_xml}")
    result_text = extract_body_text(result_xml)
    assert result_text.strip() == expected_result().strip()

def test_extract_bytes():
    extractor = Extractor()

    with open("tests/quarkus.pdf", "rb") as file:
        buffer = bytearray(file.read())
    reader, metadata = extractor.extract_bytes(buffer)

    result = read_to_string(reader)

    print(f"test_pdf:test_extract_bytes result = {result}")
    assert result == expected_result()

def test_extract_bytes_as_xml():
    extractor = Extractor()
    extractor = extractor.set_xml_output(True)

    with open("tests/quarkus.pdf", "rb") as file:
        buffer = bytearray(file.read())
    reader, metadata = extractor.extract_bytes(buffer)

    result_xml = read_to_string(reader)

    print(f"test_pdf:test_extract_bytes_as_xml result = {result_xml}")
    result_text = extract_body_text(result_xml)
    assert result_text.strip() == expected_result().strip()
