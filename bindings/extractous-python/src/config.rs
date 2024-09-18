use pyo3::{pyclass, pymethods, PyResult};

use crate::ecore;

/// OCR Strategy for PDF parsing
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PdfOcrStrategy {
    NO_OCR,
    OCR_ONLY,
    OCR_AND_TEXT_EXTRACTION,
    AUTO,
}

impl From<PdfOcrStrategy> for ecore::PdfOcrStrategy {
    fn from(strategy: PdfOcrStrategy) -> Self {
        match strategy {
            PdfOcrStrategy::NO_OCR => ecore::PdfOcrStrategy::NO_OCR,
            PdfOcrStrategy::OCR_ONLY => ecore::PdfOcrStrategy::OCR_ONLY,
            PdfOcrStrategy::OCR_AND_TEXT_EXTRACTION => ecore::PdfOcrStrategy::OCR_AND_TEXT_EXTRACTION,
            PdfOcrStrategy::AUTO => ecore::PdfOcrStrategy::AUTO,
        }
    }
}

/// PDF parsing configuration settings
///
/// These settings are used to configure the behavior of the PDF parsing.
#[pyclass]
#[derive(Clone, PartialEq)]
pub struct PdfParserConfig(ecore::PdfParserConfig);

impl From<PdfParserConfig> for ecore::PdfParserConfig {
    fn from(config: PdfParserConfig) -> Self {
        config.0
    }
}


#[pymethods]
impl PdfParserConfig {
    /// Creates a new instance of PdfParserConfig with default settings.
    #[new]
    pub fn new() -> Self {
        Self(ecore::PdfParserConfig::new())
    }

    /// Sets the OCR strategy for PDF parsing.
    /// Default: AUTO.
    pub fn set_ocr_strategy(&self, val: PdfOcrStrategy) -> PyResult<Self> {
        let inner = self.0.clone().set_ocr_strategy(val.into());
        Ok(Self(inner))
    }

    /// If true, extract the literal inline embedded OBXImages. Beware: some PDF documents of
    /// modest  size (~4MB) can contain thousands of embedded images totaling > 2.5 GB.
    /// Also, there can be surprisingly large memory consumption
    /// and/ or out of memory errors. Along the same lines, note that this does not extract
    /// "logical" images. Some PDF writers break up a single logical image into hundreds of
    /// little images. With this option set to true, you might get those hundreds of little images.
    /// NOTE ALSO: this extracts the raw images without clipping, rotation, masks, color inversion,
    /// etc. The images that this extracts may look nothing like what a human would expect given
    /// the appearance of the PDF. Set to true only with the greatest caution.
    /// Default: false.
    pub fn set_extract_inline_images(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_extract_inline_images(val);
        Ok(Self(inner))
    }

    /// Multiple pages within a PDF file might refer to the same underlying image.
    /// If extractUniqueInlineImagesOnly is set to false, the parser will call the EmbeddedExtractor
    /// each time the image appears on a page. This might be desired for some use cases. However,
    /// to avoid duplication of extracted images, set this to true. The default is true.
    /// Note that uniqueness is determined only by the underlying PDF COSObject id, not by file hash
    /// or similar equality metric. If the PDF actually contains multiple copies of the same
    /// image -- all with different object ids -- then all images will be extracted.
    /// For this parameter to have any effect, extractInlineImages must be set to true.
    /// Default: true.
    pub fn set_extract_unique_inline_images_only(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_extract_unique_inline_images_only(val);
        Ok(Self(inner))
    }

    /// If the PDF contains marked content, try to extract text and its marked structure.
    /// Default: false.
    pub fn set_extract_marked_content(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_extract_marked_content(val);
        Ok(Self(inner))
    }

    /// If the PDF contains annotations, try to extract the text of the annotations.
    /// Default: true.
    pub fn set_extract_annotation_text(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_extract_annotation_text(val);
        Ok(Self(inner))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// Microsoft Office parser configuration settings
///
/// These settings are used to configure the behavior of the MSOffice parsing.
#[pyclass]
#[derive(Clone, PartialEq)]
pub struct OfficeParserConfig(ecore::OfficeParserConfig);

impl From<OfficeParserConfig> for ecore::OfficeParserConfig {
    fn from(config: OfficeParserConfig) -> Self {
        config.0
    }
}

#[pymethods]
impl OfficeParserConfig {
    /// Creates a new instance of OfficeParserConfig with default settings.
    #[new]
    pub fn new() -> Self {
        Self(ecore::OfficeParserConfig::new())
    }

    /// Sets whether MSOffice parsers should extract macros.
    /// Default: false.
    pub fn set_extract_macros(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_extract_macros(val);
        Ok(Self(inner))
    }

    /// Sets whether the docx parser should include deleted content.
    /// Default: false.
    pub fn set_include_deleted_content(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_deleted_content(val);
        Ok(Self(inner))
    }

    /// With track changes on for the docx parser, when a section is moved, the content is stored in
    /// both the "moveFrom" section and in the "moveTo" section. If you'd like to include the
    /// section both in its original location (moveFrom) and in its new location (moveTo),
    /// set this to true. Default: false
    pub fn set_include_move_from_content(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_move_from_content(val);
        Ok(Self(inner))
    }

    /// In Excel and Word, there can be text stored within drawing shapes.
    /// (In PowerPoint everything is in a Shape) If you'd like to skip processing these to look
    /// for text, set this to false
    /// Default: true
    pub fn set_include_shape_based_content(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_shape_based_content(val);
        Ok(Self(inner))
    }


    /// Whether to include headers and footers. This only operates on headers and footers in
    /// Word and Excel, not master slide content in PowerPoint.
    /// Default: true
    pub fn set_include_headers_and_footers(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_headers_and_footers(val);
        Ok(Self(inner))
    }

    /// For table-like formats, and tables within other formats, should missing rows in sparse
    /// tables be output where detected? The default is to only output rows defined within the
    /// file, which avoid lots of blank lines, but means layout isn't preserved.
    /// Default: false
    pub fn set_include_missing_rows(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_missing_rows(val);
        Ok(Self(inner))
    }

    /// Whether to process slide notes content. If set to false, the parser will skip the text
    /// content and all embedded objects from the slide notes in ppt and pptxm.
    /// Default: true
    pub fn set_include_slide_notes(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_slide_notes(val);
        Ok(Self(inner))
    }

    /// Whether to include contents from any of the three types of masters -- slide, notes,
    /// handout -- in a .ppt or pptxm file. If set to false, the parser will not extract text
    /// or embedded objects from any of the masters.
    /// Default: true
    pub fn set_include_slide_master_content(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_include_slide_master_content(val);
        Ok(Self(inner))
    }

    /// Microsoft Excel files can sometimes contain phonetic (furigana) strings.
    /// This sets whether the parser will concatenate the phonetic runs to the original text.
    /// This is currently only supported by the xls and xlsx parsers (not the xlsb parser).
    /// Default: true.
    pub fn set_concatenate_phonetic_runs(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_concatenate_phonetic_runs(val);
        Ok(Self(inner))
    }

    /// Some .msg files can contain body content in html, rtf and/or text. The default behavior
    /// is to pick the first non-null value and include only that. If you'd like to extract all
    /// non-null body content, which is likely duplicative, set this value to true.
    /// Default: false
    pub fn set_extract_all_alternatives_from_msg(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_extract_all_alternatives_from_msg(val);
        Ok(Self(inner))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}



/// Tesseract OCR configuration settings
///
/// These settings are used to configure the behavior of the optical image recognition.
#[pyclass]
#[derive(Clone, PartialEq)]
pub struct TesseractOcrConfig(ecore::TesseractOcrConfig);

impl From<TesseractOcrConfig> for ecore::TesseractOcrConfig {
    fn from(config: TesseractOcrConfig) -> Self {
        config.0
    }
}

#[pymethods]
impl TesseractOcrConfig {
    /// Creates a new instance of TesseractOcrConfig with default settings.
    #[new]
    pub fn new() -> Self {
        Self(ecore::TesseractOcrConfig::new())
    }

    /// Sets whether Tesseract should apply rotation to the image before OCR.
    /// Default: false.
    pub fn set_apply_rotation(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_apply_rotation(val);
        Ok(Self(inner))
    }

    /// Sets the DPI (dots per inch) of the image to be processed.
    /// Default: 300.
    pub fn set_density(&self, val: i32) -> PyResult<Self> {
        let inner = self.0.clone().set_density(val);
        Ok(Self(inner))
    }

    /// Sets the color depth of the image to be processed.
    /// Default: 8.
    pub fn set_depth(&self, val: i32) -> PyResult<Self> {
        let inner = self.0.clone().set_depth(val);
        Ok(Self(inner))
    }

    /// Sets whether Tesseract should enable image preprocessing.
    /// Default: false.
    pub fn set_enable_image_preprocessing(&self, val: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_enable_image_preprocessing(val);
        Ok(Self(inner))
    }

    /// Sets the tesseract language dictionary to be used for OCR.
    /// Languages are nominally an [ISO-639-2 codes](https://en.wikipedia.org/wiki/List_of_ISO_639-2_codes).
    /// Multiple languages may be specified, separated by plus characters. e.g.
    /// "chi_tra+chi_sim+script/Arabic"
    /// Default: "eng".
    pub fn set_language(&self, val: &str) -> PyResult<Self> {
        let inner = self.0.clone().set_language(val);
        Ok(Self(inner))
    }

    /// Sets the maximum time in seconds that Tesseract should spend on OCR.
    /// Default: 120.
    pub fn set_timeout_seconds(&self, val: i32) -> PyResult<Self> {
        let inner = self.0.clone().set_timeout_seconds(val);
        Ok(Self(inner))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}