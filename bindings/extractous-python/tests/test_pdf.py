from extractous import Extractor


def expected_result():
    return "\nHello Quarkus\n\n\n"


def test_extract_file_to_dict():
    extractor = Extractor()
    ext_dict = extractor.extract_file_to_dict("tests/quarkus.pdf")
    content = ext_dict.get("content")
    metadata = ext_dict.get("metadata")

    print(content)
    assert content == expected_result()

    print(metadata)
    assert len(metadata) > 0


def test_extract_file():
    extractor = Extractor()
    reader = extractor.extract_file("tests/quarkus.pdf")

    result = ""
    b = reader.read(4096)
    while len(b) > 0:
        result += b.decode("utf-8")
        b = reader.read(4096)

    print(result)
    assert result == expected_result()
