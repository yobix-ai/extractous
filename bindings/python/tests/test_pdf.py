
from extractrs import extract


def test_pdf():
    result = extract("tests/quarkus.pdf")
    print("")
    print(result)
    assert result == "\nHello Quarkus\n\n\n"