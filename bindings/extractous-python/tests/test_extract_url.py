from extractous import Extractor
from utils import read_to_string

def test_extract_url_to_stream():
    extractor = Extractor()

    reader, metadata  = extractor.extract_url("https://www.google.com")
    result = read_to_string(reader)

    assert "Google" in result
    assert len(metadata.keys()) > 0

def test_extract_url_to_string():
    extractor = Extractor()

    content, metadata  = extractor.extract_url_to_string("https://www.google.com")

    assert "Google" in content
    assert len(metadata.keys()) > 0

def test_extract_url_to_xml():
    extractor = Extractor()
    extractor = extractor.set_parse_string_as_xml(True)

    content, metadata  = extractor.extract_url_to_string("https://www.google.com")

    assert "Google" in content
    assert len(metadata.keys()) > 0
