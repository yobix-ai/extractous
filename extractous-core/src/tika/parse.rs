use std::sync::OnceLock;

use crate::errors::ExtractResult;
use crate::tika::jni_utils::*;
use crate::tika::wrappers::*;
use crate::{
    CharSet, Metadata, OfficeParserConfig, PdfParserConfig, StreamReader, TesseractOcrConfig,
};
use jni::objects::JValue;
use jni::{AttachGuard, JavaVM};

/// Returns a reference to the shared VM isolate
/// Instead of creating a new VM for every tika call, we create a single VM that is shared
/// throughout the application.
pub(crate) fn vm() -> &'static JavaVM {
    // static items do not call `Drop` on program termination
    static GRAAL_VM: OnceLock<JavaVM> = OnceLock::new();
    GRAAL_VM.get_or_init(create_vm_isolate)
}

fn get_vm_attach_current_thread<'local>() -> ExtractResult<AttachGuard<'local>> {
    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let env = vm().attach_current_thread()?;
    Ok(env)
}

fn parse_to_stream(
    mut env: AttachGuard,
    data_source_val: JValue,
    char_set: &CharSet,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
    method_name: &str,
    signature: &str,
) -> ExtractResult<(StreamReader, Metadata)> {
    let charset_name_val = jni_new_string_as_jvalue(&mut env, &char_set.to_string())?;
    let j_pdf_conf = JPDFParserConfig::new(&mut env, pdf_conf)?;
    let j_office_conf = JOfficeParserConfig::new(&mut env, office_conf)?;
    let j_ocr_conf = JTesseractOcrConfig::new(&mut env, ocr_conf)?;

    // Make the java parse call
    let call_result = jni_call_static_method(
        &mut env,
        "ai/yobix/TikaNativeMain",
        method_name,
        signature,
        &[
            data_source_val,
            (&charset_name_val).into(),
            (&j_pdf_conf.internal).into(),
            (&j_office_conf.internal).into(),
            (&j_ocr_conf.internal).into(),
            JValue::Bool(if as_xml { 1 } else { 0 }),
        ],
    );
    let call_result_obj = call_result?.l()?;

    // Create and process the JReaderResult
    let result = JReaderResult::new(&mut env, call_result_obj)?;
    let j_reader = JReaderInputStream::new(&mut env, result.java_reader)?;

    Ok((StreamReader { inner: j_reader }, result.metadata))
}

pub fn parse_file(
    file_path: &str,
    char_set: &CharSet,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool
) -> ExtractResult<(StreamReader, Metadata)> {
    let mut env = get_vm_attach_current_thread()?;

    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;
    parse_to_stream(
        env,
        (&file_path_val).into(),
        char_set,
        pdf_conf,
        office_conf,
        ocr_conf,
        as_xml,
        "parseFile",
        "(Ljava/lang/String;\
        Ljava/lang/String;\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        Z\
        )Lai/yobix/ReaderResult;",
    )
}

pub fn parse_bytes(
    buffer: &[u8],
    char_set: &CharSet,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
) -> ExtractResult<(StreamReader, Metadata)> {
    let mut env = get_vm_attach_current_thread()?;

    // Because we know the buffer is used for reading only, cast it to *mut u8 to satisfy the
    // jni_new_direct_buffer call, which requires a mutable pointer
    let mut_ptr: *mut u8 = buffer.as_ptr() as *mut u8;

    let byte_buffer = jni_new_direct_buffer(&mut env, mut_ptr, buffer.len())?;

    parse_to_stream(
        env,
        (&byte_buffer).into(),
        char_set,
        pdf_conf,
        office_conf,
        ocr_conf,
        as_xml,
        "parseBytes",
        "(Ljava/nio/ByteBuffer;\
        Ljava/lang/String;\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        Z\
        )Lai/yobix/ReaderResult;",
    )
}

pub fn parse_url(
    url: &str,
    char_set: &CharSet,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
) -> ExtractResult<(StreamReader, Metadata)> {
    let mut env = get_vm_attach_current_thread()?;

    let url_val = jni_new_string_as_jvalue(&mut env, url)?;
    parse_to_stream(
        env,
        (&url_val).into(),
        char_set,
        pdf_conf,
        office_conf,
        ocr_conf,
        as_xml,
        "parseUrl",
        "(Ljava/lang/String;\
        Ljava/lang/String;\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        Z\
        )Lai/yobix/ReaderResult;",
    )
}

/// Parses a file to a JStringResult using the Apache Tika library.
pub fn parse_to_string(
    mut env: AttachGuard,
    data_source_val: JValue,
    max_length: i32,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
    method_name: &str,
    signature: &str,
) -> ExtractResult<(String, Metadata)> {
    let j_pdf_conf = JPDFParserConfig::new(&mut env, pdf_conf)?;
    let j_office_conf = JOfficeParserConfig::new(&mut env, office_conf)?;
    let j_ocr_conf = JTesseractOcrConfig::new(&mut env, ocr_conf)?;

    let call_result = jni_call_static_method(
        &mut env,
        "ai/yobix/TikaNativeMain",
        method_name,
        signature,
        &[
            data_source_val,
            JValue::Int(max_length),
            (&j_pdf_conf.internal).into(),
            (&j_office_conf.internal).into(),
            (&j_ocr_conf.internal).into(),
            JValue::Bool(if as_xml { 1 } else { 0 }),
        ],
    );
    let call_result_obj = call_result?.l()?;

    // Create and process the JStringResult
    let result = JStringResult::new(&mut env, call_result_obj)?;
    Ok((result.content, result.metadata))
}

/// Parses a file to a string using the Apache Tika library.
pub fn parse_file_to_string(
    file_path: &str,
    max_length: i32,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
) -> ExtractResult<(String, Metadata)> {
    let mut env = get_vm_attach_current_thread()?;

    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;
    parse_to_string(
        env,
        (&file_path_val).into(),
        max_length,
        pdf_conf,
        office_conf,
        ocr_conf,
        as_xml,
        "parseFileToString",
        "(Ljava/lang/String;\
        I\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        Z\
        )Lai/yobix/StringResult;",
    )
}

/// Parses bytes to a string using the Apache Tika library.
pub fn parse_bytes_to_string(
    buffer: &[u8],
    max_length: i32,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
) -> ExtractResult<(String, Metadata)> {
    let mut env = get_vm_attach_current_thread()?;

    // Because we know the buffer is used for reading only, cast it to *mut u8 to satisfy the
    // jni_new_direct_buffer call, which requires a mutable pointer
    let mut_ptr: *mut u8 = buffer.as_ptr() as *mut u8;

    let byte_buffer = jni_new_direct_buffer(&mut env, mut_ptr, buffer.len())?;

    parse_to_string(
        env,
        (&byte_buffer).into(),
        max_length,
        pdf_conf,
        office_conf,
        ocr_conf,
        as_xml,
        "parseBytesToString",
        "(Ljava/nio/ByteBuffer;\
        I\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        Z\
        )Lai/yobix/StringResult;",
    )
}

/// Parses a url to a string using the Apache Tika library.
pub fn parse_url_to_string(
    url: &str,
    max_length: i32,
    pdf_conf: &PdfParserConfig,
    office_conf: &OfficeParserConfig,
    ocr_conf: &TesseractOcrConfig,
    as_xml: bool,
) -> ExtractResult<(String, Metadata)> {
    let mut env = get_vm_attach_current_thread()?;

    let url_val = jni_new_string_as_jvalue(&mut env, url)?;
    parse_to_string(
        env,
        (&url_val).into(),
        max_length,
        pdf_conf,
        office_conf,
        ocr_conf,
        as_xml,
        "parseUrlToString",
        "(Ljava/lang/String;\
        I\
        Lorg/apache/tika/parser/pdf/PDFParserConfig;\
        Lorg/apache/tika/parser/microsoft/OfficeParserConfig;\
        Lorg/apache/tika/parser/ocr/TesseractOCRConfig;\
        Z\
        )Lai/yobix/StringResult;",
    )
}
