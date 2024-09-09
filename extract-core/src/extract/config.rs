#[derive(Debug)]
pub struct PdfParserConfig {
    pub extract_inline_images: bool,
    pub extract_marked_content: bool,
}

impl Default for PdfParserConfig {
    fn default() -> Self {
        Self {
            extract_inline_images: true,
            extract_marked_content: false,
        }
    }
}

impl PdfParserConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_extract_inline_images(&mut self, val: bool) -> &mut Self {
        self.extract_inline_images = val;
        self
    }
    pub fn set_extract_marked_content(&mut self, val: bool) -> &mut Self {
        self.extract_marked_content = val;
        self
    }
}

#[derive(Debug, Default)]
pub struct OfficeParserConfig {
    concatenate_phonetic_runs: bool,
    extract_all_alternatives_from_msg: bool,
    extract_macros: bool,
    include_deleted_content: bool,
    include_headers_and_footers: bool,
    include_missing_rows: bool,
}

#[derive(Debug, Default)]
pub struct TesseractOcrConfig {
    apply_rotation: bool,
    density: i32,
    depth: i32,
    enable_image_preprocessing: bool,
    language: String,
}
