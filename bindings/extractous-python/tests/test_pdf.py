from extractous import Extractor


def expected_result():
    return "\nHello Quarkus\n\n\n"


def test_extract_file_to_string():
    extractor = Extractor()
    result = extractor.extract_file_to_string("tests/quarkus.pdf")

    print(result)
    assert result == expected_result()


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