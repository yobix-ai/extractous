import pytest

from extractous import Extractor
from sklearn.feature_extraction.text import CountVectorizer
from sklearn.metrics.pairwise import cosine_similarity as cosine_sim

TEST_CASES = [
    ("2022_Q3_AAPL.pdf", 0.9),
    ("science-exploration-1p.pptx", 0.9),
    ("simple.odt", 0.9),
    ("table-multi-row-column-cells-actual.csv", 0.9),
    ("vodafone.xlsx", 0.4),
    ("category-level.docx", 0.9),
    ("simple.doc", 0.9),
    ("simple.pptx", 0.9),
    ("table-multi-row-column-cells.png", -1.0),
    ("winter-sports.epub", 0.9),
]

@pytest.mark.parametrize("file_name, target_dist", TEST_CASES)
def test_extract_file_data(file_name, target_dist):
    """Test the extraction and comparison of various file types."""
    original_filepath = f"../../test_files/documents/{file_name}"
    expected_result_filepath = f"../../test_files/expected_result/{file_name}.txt"
    expected_metadata_result_filepath = f"../../test_files/expected_result/{file_name}.metadata.txt"
    extractor = Extractor()
    ext_dict = extractor.extract_file_to_dict(original_filepath)
    content = ext_dict.get("content")
    metadata = ext_dict.get("metadata")
    metadata.sort()
    with open(expected_result_filepath, "r") as file:
        expected = file.read()
    with open(expected_metadata_result_filepath, 'r') as file:
            expected_metadata = [line.strip() for line in file.readlines()]

    assert cosine_similarity(content, expected) > target_dist, \
        f"Cosine similarity is less than {target_dist} for file: {file_name}"

    assert len(metadata) > 0
    assert set(metadata).issubset(expected_metadata)


def cosine_similarity(text1, text2):
    """Calculate the cosine similarity between two texts."""

    # Create the CountVectorizer and transform the texts into vectors
    vectorizer = CountVectorizer().fit_transform([text1, text2])
    vectors = vectorizer.toarray()

    # Calculate cosine similarity between the two vectors
    cos_sim = cosine_sim(vectors)
    return cos_sim[0][1]

def read_metadata_array(file_path):

    return lines
