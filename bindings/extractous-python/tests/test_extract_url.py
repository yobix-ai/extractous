from extractous import Extractor
from utils import read_to_string

def test_extract_url():
    extractor = Extractor()

    reader = extractor.extract_url("https://www.google.com")
    result = read_to_string(reader)

    assert "Google" in result

def test_extract_url_with_metadata():
    extractor = Extractor()
    _reader, metadata = extractor.extract_url_with_metadata("https://www.google.com")
    assert len(metadata.keys()) > 0
