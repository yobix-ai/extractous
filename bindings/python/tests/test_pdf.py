
from extractrs import extract


def test_pdf():
    result = extract("tests/quarkus.pdf")
    assert result == "\nHello Quarkus\n\n\n"