from extractous import Extractor
from utils import read_to_string

def test_extract_url():
    extractor = Extractor()

    reader = extractor.extract_url("https://www.google.com")
    result = read_to_string(reader)

    assert "Google" in result
