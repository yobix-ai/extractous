use strum_macros::{Display, EnumString};

/// OCR Strategy for PDF parsing
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[allow(non_camel_case_types)]
pub enum PdfOcrStrategy {
    NO_OCR,
    OCR_ONLY,
    OCR_AND_TEXT_EXTRACTION,
    #[default]
    AUTO,
}

/// PDF parsing configuration settings
///
/// These settings are used to configure the behavior of the PDF parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct PdfParserConfig {
    pub(crate) ocr_strategy: PdfOcrStrategy,
    pub(crate) extract_inline_images: bool,
    pub(crate) extract_unique_inline_images_only: bool,
    pub(crate) extract_marked_content: bool,
    pub(crate) extract_annotation_text: bool,
}

impl Default for PdfParserConfig {
    fn default() -> Self {
        Self {
            ocr_strategy: PdfOcrStrategy::AUTO,
            extract_inline_images: false,
            extract_unique_inline_images_only: false,
            extract_marked_content: false,
            extract_annotation_text: true,
        }
    }
}

impl PdfParserConfig {
    /// Creates a new instance of PdfParserConfig with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the OCR strategy for PDF parsing.
    /// Default: AUTO.
    pub fn set_ocr_strategy(mut self, val: PdfOcrStrategy) -> Self {
        self.ocr_strategy = val;
        self
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
    pub fn set_extract_inline_images(mut self, val: bool) -> Self {
        self.extract_inline_images = val;
        self
    }

    /// Multiple pages within a PDF file might refer to the same underlying image.
    /// If extractUniqueInlineImagesOnly is set to false, the parser will call the EmbeddedExtractor
    /// each time the image appears on a page. This might be desired for some use cases. However,
    /// to avoid duplication of extracted images, set this to true. The default is true.
    /// Note that uniqueness is determined only by the underlying PDF COSObject id, not by file hash
    /// or similar equality metric. If the PDF actually contains multiple copies of the same
    /// image -- all with different object ids -- then all images will be extracted.
    /// For this parameter to have any effect, extractInlineImages must be set to true.
    /// Default: false.
    pub fn set_extract_unique_inline_images_only(mut self, val: bool) -> Self {
        self.extract_unique_inline_images_only = val;
        self
    }

    /// If the PDF contains marked content, try to extract text and its marked structure.
    /// Default: false.
    pub fn set_extract_marked_content(mut self, val: bool) -> Self {
        self.extract_marked_content = val;
        self
    }

    /// If the PDF contains annotations, try to extract the text of the annotations.
    /// Default: true.
    pub fn set_extract_annotation_text(mut self, val: bool) -> Self {
        self.extract_annotation_text = val;
        self
    }
}

/// Microsoft Office parser configuration settings
///
/// These settings are used to configure the behavior of the MSOffice parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct OfficeParserConfig {
    pub(crate) extract_macros: bool,
    pub(crate) include_deleted_content: bool,
    pub(crate) include_move_from_content: bool,
    pub(crate) include_shape_based_content: bool,
    pub(crate) include_headers_and_footers: bool,
    pub(crate) include_missing_rows: bool,
    pub(crate) include_slide_notes: bool,
    pub(crate) include_slide_master_content: bool,
    pub(crate) concatenate_phonetic_runs: bool,
    pub(crate) extract_all_alternatives_from_msg: bool,
}

impl Default for OfficeParserConfig {
    fn default() -> Self {
        Self {
            extract_macros: false,
            include_deleted_content: false,
            include_move_from_content: false,
            include_shape_based_content: true,
            include_headers_and_footers: false,
            include_missing_rows: false,
            include_slide_notes: true,
            include_slide_master_content: true,
            concatenate_phonetic_runs: true,
            extract_all_alternatives_from_msg: false,
        }
    }
}

impl OfficeParserConfig {
    /// Creates a new instance of OfficeParserConfig with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether MSOffice parsers should extract macros.
    /// Default: false.
    pub fn set_extract_macros(mut self, val: bool) -> Self {
        self.extract_macros = val;
        self
    }

    /// Sets whether the docx parser should include deleted content.
    /// Default: false.
    pub fn set_include_deleted_content(mut self, val: bool) -> Self {
        self.include_deleted_content = val;
        self
    }

    /// With track changes on for the docx parser, when a section is moved, the content is stored in
    /// both the "moveFrom" section and in the "moveTo" section. If you'd like to include the
    /// section both in its original location (moveFrom) and in its new location (moveTo),
    /// set this to true. Default: false
    pub fn set_include_move_from_content(mut self, val: bool) -> Self {
        self.include_move_from_content = val;
        self
    }

    /// In Excel and Word, there can be text stored within drawing shapes.
    /// (In PowerPoint everything is in a Shape) If you'd like to skip processing these to look
    /// for text, set this to false
    /// Default: true
    pub fn set_include_shape_based_content(mut self, val: bool) -> Self {
        self.include_shape_based_content = val;
        self
    }

    /// Whether to include headers and footers. This only operates on headers and footers in
    /// Word and Excel, not master slide content in PowerPoint.
    /// Default: true
    pub fn set_include_headers_and_footers(mut self, val: bool) -> Self {
        self.include_headers_and_footers = val;
        self
    }

    /// For table-like formats, and tables within other formats, should missing rows in sparse
    /// tables be output where detected? The default is to only output rows defined within the
    /// file, which avoid lots of blank lines, but means layout isn't preserved.
    /// Default: false
    pub fn set_include_missing_rows(mut self, val: bool) -> Self {
        self.include_missing_rows = val;
        self
    }

    /// Whether to process slide notes content. If set to false, the parser will skip the text
    /// content and all embedded objects from the slide notes in ppt and pptxm.
    /// Default: true
    pub fn set_include_slide_notes(mut self, val: bool) -> Self {
        self.include_slide_notes = val;
        self
    }

    /// Whether to include contents from any of the three types of masters -- slide, notes,
    /// handout -- in a .ppt or pptxm file. If set to false, the parser will not extract text
    /// or embedded objects from any of the masters.
    /// Default: true
    pub fn set_include_slide_master_content(mut self, val: bool) -> Self {
        self.include_slide_master_content = val;
        self
    }

    /// Microsoft Excel files can sometimes contain phonetic (furigana) strings.
    /// This sets whether the parser will concatenate the phonetic runs to the original text.
    /// This is currently only supported by the xls and xlsx parsers (not the xlsb parser).
    /// Default: true.
    pub fn set_concatenate_phonetic_runs(mut self, val: bool) -> Self {
        self.concatenate_phonetic_runs = val;
        self
    }

    /// Some .msg files can contain body content in html, rtf and/or text. The default behavior
    /// is to pick the first non-null value and include only that. If you'd like to extract all
    /// non-null body content, which is likely duplicative, set this value to true.
    /// Default: false
    pub fn set_extract_all_alternatives_from_msg(mut self, val: bool) -> Self {
        self.extract_all_alternatives_from_msg = val;
        self
    }
}

/// Tesseract OCR configuration settings
///
/// These settings are used to configure the behavior of the optical image recognition.
#[derive(Debug, Clone, PartialEq)]
pub struct TesseractOcrConfig {
    pub(crate) density: i32,
    pub(crate) depth: i32,
    pub(crate) timeout_seconds: i32,
    pub(crate) enable_image_preprocessing: bool,
    pub(crate) apply_rotation: bool,
    pub(crate) language: String,
}

impl Default for TesseractOcrConfig {
    fn default() -> Self {
        Self {
            density: 300,
            depth: 4,
            timeout_seconds: 130,
            enable_image_preprocessing: false,
            apply_rotation: false,
            language: "eng".to_string(),
        }
    }
}

impl TesseractOcrConfig {
    /// Creates a new instance of TesseractOcrConfig with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether Tesseract should apply rotation to the image before OCR.
    /// Default: false.
    pub fn set_apply_rotation(mut self, val: bool) -> Self {
        self.apply_rotation = val;
        self
    }

    /// Sets the DPI (dots per inch) of the image to be processed.
    /// Default: 300.
    pub fn set_density(mut self, val: i32) -> Self {
        self.density = val;
        self
    }

    /// Sets the color depth of the image to be processed.
    /// Default: 8.
    pub fn set_depth(mut self, val: i32) -> Self {
        self.depth = val;
        self
    }

    /// Sets whether Tesseract should enable image preprocessing.
    /// Default: false.
    pub fn set_enable_image_preprocessing(mut self, val: bool) -> Self {
        self.enable_image_preprocessing = val;
        self
    }

    /// Sets the tesseract language dictionary to be used for OCR.
    /// Languages are nominally an [ISO-639-2 codes](https://en.wikipedia.org/wiki/List_of_ISO_639-2_codes).
    /// Multiple languages may be specified, separated by plus characters. e.g.
    /// "chi_tra+chi_sim+script/Arabic"
    /// Default: "eng".
    pub fn set_language(mut self, val: &str) -> Self {
        self.language = val.to_string();
        self
    }

    /// Sets the maximum time in seconds that Tesseract should spend on OCR.
    /// Default: 120.
    pub fn set_timeout_seconds(mut self, val: i32) -> Self {
        self.timeout_seconds = val;
        self
    }
}
