/// Pdf parser configuration settings
///
/// These settings are used to configure the behavior of the PDF parsing.
pub struct PdfParserConfig {
    pub extract_inline_images: bool,
    pub extract_marked_content: bool,
    pub extract_annotation_text: bool,
}

impl Default for PdfParserConfig {
    fn default() -> Self {
        Self {
            extract_inline_images: false,
            extract_marked_content: false,
            extract_annotation_text: true,
        }
    }
}

impl PdfParserConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// If true, extract the literal inline embedded OBXImages. Beware: some PDF documents of
    /// modest  size (~4MB) can contain thousands of embedded images totaling > 2.5 GB.
    /// Also, at least as of PDFBox 1.8.5, there can be surprisingly large memory consumption
    /// and/ or out of memory errors. Along the same lines, note that this does not extract
    /// "logical" images. Some PDF writers break up a single logical image into hundreds of
    /// little images. With this option set to true, you might get those hundreds of little images.
    /// NOTE ALSO: this extracts the raw images without clipping, rotation, masks, color inversion,
    /// etc. The images that this extracts may look nothing like what a human would expect given
    /// the appearance of the PDF. Set to true only with the greatest caution.
    /// Default: false.
    pub fn set_extract_inline_images(&mut self, val: bool) -> &mut Self {
        self.extract_inline_images = val;
        self
    }

    /// If the PDF contains marked content, try to extract text and its marked structure.
    /// Default: false.
    pub fn set_extract_marked_content(&mut self, val: bool) -> &mut Self {
        self.extract_marked_content = val;
        self
    }

    /// If the PDF contains annotations, try to extract the text of the annotations.
    /// Default: true.
    pub fn set_extract_annotation_text(&mut self, val: bool) -> &mut Self {
        self.extract_annotation_text = val;
        self
    }
}

/// Microsoft Office parser configuration settings
///
/// These settings are used to configure the behavior of the MSOffice parsing.
pub struct OfficeParserConfig {
    extract_macros: bool,
    include_deleted_content: bool,
    include_move_from_content: bool,
    include_shape_based_content: bool,
    include_headers_and_footers: bool,
    include_missing_rows: bool,
    include_slide_notes: bool,
    include_slide_master_content: bool,
    concatenate_phonetic_runs: bool,
    extract_all_alternatives_from_msg: bool,
}

impl Default for OfficeParserConfig {
    fn default() -> Self {
        Self {
            extract_macros: false,
            include_deleted_content: false,
            include_move_from_content: false,
            include_shape_based_content: true,
            include_headers_and_footers: true,
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
    pub fn set_extract_macros(&mut self, val: bool) -> &mut Self {
        self.extract_macros = val;
        self
    }

    /// Sets whether the docx parser should include deleted content.
    /// Default: false.
    pub fn set_include_deleted_content(&mut self, val: bool) -> &mut Self {
        self.include_deleted_content = val;
        self
    }

    /// With track changes on for the docx parser, when a section is moved, the content is stored in
    /// both the "moveFrom" section and in the "moveTo" section. If you'd like to include the
    /// section both in its original location (moveFrom) and in its new location (moveTo),
    /// set this to true. Default: false
    pub fn set_include_move_from_content(&mut self, val: bool) -> &mut Self {
        self.include_move_from_content = val;
        self
    }

    /// In Excel and Word, there can be text stored within drawing shapes.
    /// (In PowerPoint everything is in a Shape) If you'd like to skip processing these to look
    /// for text, set this to false
    /// Default: true
    pub fn set_include_shape_based_content(&mut self, val: bool) -> &mut Self {
        self.include_shape_based_content = val;
        self
    }

    /// Whether to include headers and footers. This only operates on headers and footers in
    /// Word and Excel, not master slide content in PowerPoint.
    /// Default: true
    pub fn set_include_headers_and_footers(&mut self, val: bool) -> &mut Self {
        self.include_headers_and_footers = val;
        self
    }

    /// For table-like formats, and tables within other formats, should missing rows in sparse
    /// tables be output where detected? The default is to only output rows defined within the
    /// file, which avoid lots of blank lines, but means layout isn't preserved.
    /// Default: false
    pub fn set_include_missing_rows(&mut self, val: bool) -> &mut Self {
        self.include_missing_rows = val;
        self
    }

    /// Whether to process slide notes content. If set to false, the parser will skip the text
    /// content and all embedded objects from the slide notes in ppt and pptxm.
    /// Default: true
    pub fn set_include_slide_notes(&mut self, val: bool) -> &mut Self {
        self.include_slide_notes = val;
        self
    }

    /// Whether to include contents from any of the three types of masters -- slide, notes,
    /// handout -- in a .ppt or pptxm file. If set to false, the parser will not extract text
    /// or embedded objects from any of the masters.
    /// Default: true
    pub fn set_include_slide_master_content(&mut self, val: bool) -> &mut Self {
        self.include_slide_master_content = val;
        self
    }

    /// Microsoft Excel files can sometimes contain phonetic (furigana) strings.
    /// This sets whether the parser will concatenate the phonetic runs to the original text.
    /// This is currently only supported by the xls and xlsx parsers (not the xlsb parser).
    /// Default: true.
    pub fn set_concatenate_phonetic_runs(&mut self, val: bool) -> &mut Self {
        self.concatenate_phonetic_runs = val;
        self
    }

    /// Some .msg files can contain body content in html, rtf and/or text. The default behavior
    /// is to pick the first non-null value and include only that. If you'd like to extract all
    /// non-null body content, which is likely duplicative, set this value to true.
    /// Default: false
    pub fn set_extract_all_alternatives_from_msg(&mut self, val: bool) -> &mut Self {
        self.extract_all_alternatives_from_msg = val;
        self
    }
}

/// Tesseract OCR configuration settings
///
/// These settings are used to configure the behavior of the optical image recognition.
pub struct TesseractOcrConfig {
    apply_rotation: bool,
    density: i32,
    depth: i32,
    timeout_seconds: i32,
    enable_image_preprocessing: bool,
    language: String,
}

impl Default for TesseractOcrConfig {
    fn default() -> Self {
        Self {
            apply_rotation: false,
            density: 300,
            depth: 4,
            timeout_seconds: 120,
            enable_image_preprocessing: false,
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
    pub fn set_apply_rotation(&mut self, val: bool) -> &mut Self {
        self.apply_rotation = val;
        self
    }

    /// Sets the DPI (dots per inch) of the image to be processed.
    /// Default: 300.
    pub fn set_density(&mut self, val: i32) -> &mut Self {
        self.density = val;
        self
    }

    /// Sets the color depth of the image to be processed.
    /// Default: 8.
    pub fn set_depth(&mut self, val: i32) -> &mut Self {
        self.depth = val;
        self
    }

    /// Sets whether Tesseract should enable image preprocessing.
    /// Default: false.
    pub fn set_enable_image_preprocessing(&mut self, val: bool) -> &mut Self {
        self.enable_image_preprocessing = val;
        self
    }

    /// Sets the tesseract language dictionary to be used for OCR.
    /// Languages are nominally an ISO-639-2 codes. Multiple languages may be specified, separated
    /// by plus characters. e.g. "chi_tra+chi_sim+script/Arabic"
    /// Default: "eng".
    pub fn set_language(&mut self, val: &str) -> &mut Self {
        self.language = val.to_string();
        self
    }

    /// Sets the maximum time in seconds that Tesseract should spend on OCR.
    /// Default: 120.
    pub fn set_timeout_seconds(&mut self, val: i32) -> &mut Self {
        self.timeout_seconds = val;
        self
    }
}