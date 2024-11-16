from extractous import Extractor, PdfOcrStrategy, PdfParserConfig, TesseractOcrConfig
from utils import cosine_similarity


def test_ara_ocr_png():
    ocr_config = TesseractOcrConfig().set_language("ara")
    extractor = Extractor().set_ocr_config(ocr_config)
    result, metadata = extractor.extract_file_to_string("../../test_files/documents/ara-ocr.png")

    with open("../../test_files/expected_result/ara-ocr.png.txt", "r", encoding="utf8") as file:
        expected = file.read()

    assert cosine_similarity(result, expected) > 0.9


def test_extract_file_to_string_ocr_only_strategy_deu_ocr_pdf():
    test_file = "../../test_files/documents/deu-ocr.pdf"
    expected_result_file = "../../test_files/expected_result/deu-ocr.pdf.txt"

    pdf_config = PdfParserConfig().set_ocr_strategy(PdfOcrStrategy.OCR_ONLY)
    ocr_config = TesseractOcrConfig().set_language("deu")

    # Note builder patter is used
    extractor = Extractor()
    extractor = extractor.set_ocr_config(ocr_config)
    extractor = extractor.set_pdf_config(pdf_config)

    result, metadata = extractor.extract_file_to_string(test_file)

    with open(expected_result_file, "r", encoding="utf8") as file:
        expected = file.read()

    assert cosine_similarity(result, expected) > 0.9


def test_test_extract_file_to_string_no_ocr_strategy_deu_ocr_pdf():
    test_file = "../../test_files/documents/deu-ocr.pdf"

    pdf_config = PdfParserConfig()
    pdf_config = pdf_config.set_ocr_strategy(PdfOcrStrategy.NO_OCR)
    ocr_config = TesseractOcrConfig()
    ocr_config = ocr_config.set_language("deu")

    extractor = Extractor().set_ocr_config(ocr_config).set_pdf_config(pdf_config)

    result, metadata = extractor.extract_file_to_string(test_file)

    assert result.strip() == ""
