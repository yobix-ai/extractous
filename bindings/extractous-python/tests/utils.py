from sklearn.feature_extraction.text import CountVectorizer
from sklearn.metrics.pairwise import cosine_similarity as cosine_sim

def cosine_similarity(text1, text2):
    """Calculate the cosine similarity between two texts."""

    # Create the CountVectorizer and transform the texts into vectors
    vectorizer = CountVectorizer().fit_transform([text1, text2])
    vectors = vectorizer.toarray()

    # Calculate cosine similarity between the two vectors
    cos_sim = cosine_sim(vectors)
    return cos_sim[0][1]

def read_to_string(reader):
    """Read from stream to string."""
    result = ""
    b = reader.read(4096)
    while len(b) > 0:
        result += b.decode("utf-8")
        b = reader.read(4096)
    return result

