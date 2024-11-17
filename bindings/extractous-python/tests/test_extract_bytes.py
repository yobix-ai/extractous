import json
import pytest

from extractous import Extractor
from utils import calculate_similarity_percent, cosine_similarity, read_to_string, read_file_to_bytearray, \
    is_expected_metadata_contained

TEST_CASES = [
    ("2022_Q3_AAPL.pdf", 0.9, 0.8),
    ("science-exploration-1p.pptx", 0.9, 0.8),
    ("simple.odt", 0.9, 0.8),
    ("table-multi-row-column-cells-actual.csv", 0.9, 0.6),
    ("vodafone.xlsx", 0.4, 0.8),
    ("category-level.docx", 0.9, 0.8),
    ("simple.doc", 0.9, 0.8),
    ("simple.pptx", 0.9, 0.8),
    #("table-multi-row-column-cells.png", -1.0, 0.8),
    ("winter-sports.epub", 0.9, 0.8),
    ("bug_16.docx", 0.9, 0.8),
    #("eng-ocr.pdf", 0.9, 0.8),
]


@pytest.mark.parametrize("file_name, target_dist, metadata_dist", TEST_CASES)
def test_extract_bytes_to_string(file_name, target_dist, metadata_dist):
    """Test the extraction from bytes of various file types."""
    original_filepath = f"../../test_files/documents/{file_name}"
    expected_result_filepath = f"../../test_files/expected_result/{file_name}.txt"
    expected_metadata_result_filepath = f"../../test_files/expected_result/{file_name}.metadata.json"

    # Read expected
    with open(expected_result_filepath, "r",  encoding="utf8") as file:
        expected = file.read()
    with open(expected_metadata_result_filepath, 'r', encoding="utf8") as file:
        expected_metadata = json.load(file)

    # Extract
    file_bytes = read_file_to_bytearray(original_filepath)

    extractor = Extractor()
    result, metadata = extractor.extract_bytes_to_string(file_bytes)

    # Check Expected
    assert cosine_similarity(result, expected) > target_dist, \
        f"Cosine similarity is less than {target_dist} for file: {file_name}"

    # Check metadata
    percent_similarity = calculate_similarity_percent(metadata, expected_metadata)
    assert percent_similarity > metadata_dist, \
        f"The metadata similarity is lower than expected. Current {percent_similarity}% | filename: {file_name}"

@pytest.mark.parametrize("file_name, target_dist, metadata_dist", TEST_CASES)
def test_extract_bytes_to_stream(file_name, target_dist, metadata_dist):
    """Test the extraction from bytes of various file types."""
    original_filepath = f"../../test_files/documents/{file_name}"
    expected_result_filepath = f"../../test_files/expected_result/{file_name}.txt"
    expected_metadata_result_filepath = f"../../test_files/expected_result/{file_name}.metadata.json"

    # Read expected
    with open(expected_result_filepath, "r",  encoding="utf8") as file:
        expected = file.read()
    with open(expected_metadata_result_filepath, 'r', encoding="utf8") as file:
        expected_metadata = json.load(file)

    # Extract
    file_bytes = read_file_to_bytearray(original_filepath)

    extractor = Extractor()
    reader, metadata = extractor.extract_bytes(file_bytes)
    result = read_to_string(reader)

    # Check Expected
    assert cosine_similarity(result, expected) > target_dist, \
        f"Cosine similarity is less than {target_dist} for file: {file_name}"

    # Check metadata
    percent_similarity = calculate_similarity_percent(metadata, expected_metadata)
    assert percent_similarity > metadata_dist, \
        f"The metadata similarity is lower than expected. Current {percent_similarity}% | filename: {file_name}"