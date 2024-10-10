use std::sync::OnceLock;

use jni::objects::JValue;
use jni::JavaVM;

use crate::errors::ExtractResult;
use crate::tika::jni_utils::*;
use crate::tika::wrappers::*;
use crate::{CharSet, OfficeParserConfig, PdfParserConfig, StreamReader, TesseractOcrConfig};

/// Returns a reference to the shared VM isolate
/// Instead of creating a new VM for every tika call, we create a single VM that is shared
/// throughout the application.
pub(crate) fn vm() -> &'static JavaVM {
    // static items do not call `Drop` on program termination
    static GRAAL_VM: OnceLock<JavaVM> = OnceLock::new();
    GRAAL_VM.get_or_init(create_vm_isolate)
}

pub fn parse_file(
    file_path: &str,
    char_set: &CharSet,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
) -> ExtractResult<StreamReader> {
    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;

    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;
    let charset_name_val = jni_new_string_as_jvalue(&mut env, &char_set.to_string())?;
    let j_pdf_conf = JPDFParserConfig::new(&mut env, pdf_conf)?;
    let j_office_conf = JOfficeParserConfig::new(&mut env, office_conf)?;
    let j_ocr_conf = JTesseractOcrConfig::new(&mut env, ocr_conf)?;

    // Make the java parse call
    let call_result = jni_call_static_method(
        &mut env,
        "ai/yobix/TikaNativeMain",
        "parseFile",
        "(Ljava/lang/String;\
        Ljava/lang/String;\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        )Lai/yobix/ReaderResult;",
        &[
            (&file_path_val).into(),
            (&charset_name_val).into(),
            (&j_pdf_conf.internal).into(),
            (&j_office_conf.internal).into(),
            (&j_ocr_conf.internal).into(),
        ],
    );
    let call_result_obj = call_result?.l()?;

    // Create and process the JReaderResult
    let result = JReaderResult::new(&mut env, call_result_obj)?;
    let j_reader = JReaderInputStream::new(&mut env, result.java_reader)?;

    Ok(StreamReader { inner: j_reader })
}

/// Parses a file to a rust struct using the Apache Tika library.
pub fn parse_file_to_struct(file_path: &str, max_length: i32) -> ExtractResult<JResult> {
    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;
    // Create a new Java string from the Rust string
    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;
    let call_result = jni_call_static_method(
        &mut env,
        "ai/yobix/TikaNativeMain",
        "parseToString",
        "(Ljava/lang/String;I)Lai/yobix/StringResult;",
        &[(&file_path_val).into(), JValue::Int(max_length)],
    );
    let call_result_obj = call_result?.l()?;
    // Create and process the JStringResult
    let result = JResult::new(&mut env, call_result_obj)?;
    Ok(result)
}
