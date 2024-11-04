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