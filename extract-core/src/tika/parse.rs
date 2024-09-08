use std::sync::OnceLock;

use jni::JavaVM;
use jni::objects::JValue;
use jni::signature::ReturnType;

use crate::errors::ExtractResult;
use crate::PdfParserConfig;
use crate::tika::jni_utils::*;
use crate::tika::wrappers::*;

/// Returns a reference to the shared VM isolate
/// Instead of creating a new VM for every tika call, we create a single VM that is shared
/// throughout the application.
pub(crate) fn vm() -> &'static JavaVM {
    // static items do not call `Drop` on program termination
    static GRAAL_VM: OnceLock<JavaVM> = OnceLock::new();
    GRAAL_VM.get_or_init(|| {
        create_vm_isolate()
    })
}



pub fn parse_file<'local>(
    file_path: &str,
    pdf_conf: &'local PdfParserConfig,
) -> ExtractResult<JReaderInputStream<'local>> {
    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;

    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;

    let jpdf_conf = JPDFParserConfig::new(&mut env, pdf_conf)?;

    // Make the java parse call
    let call_result = env.call_static_method(
        "ai/yobix/TikaNativeMain",
        "parsePdf",
        "(Ljava/lang/String;Lorg/apache/tika/parser/pdf/PDFParserConfig;)Lai/yobix/ReaderResult;",
        &[(&file_path_val).into(), (&jpdf_conf.internal).into()],
    );
    jni_check_exception(&mut env)?; // prints any exceptions thrown to stderr
    let call_result_obj = call_result?.l()?;

    // Create and process the JReaderResult
    let result = JReaderResult::new(&mut env, call_result_obj)?;

    Ok(JReaderInputStream::new(result.java_reader))
}

/// Parses a file to a string using the Apache Tika library.
pub fn parse_file_to_string(file_path: &str, max_length: i32) -> ExtractResult<String> {
    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;

    // Create a new Java string from the Rust string
    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;

    // Make the parse call
    let main_class = env.find_class("ai/yobix/TikaNativeMain")?;
    let parse_mid = env.get_static_method_id(
        &main_class,
        "parseToString",
        "(Ljava/lang/String;I)Lai/yobix/StringResult;",
    )?;
    let call_result = unsafe {
        env.call_static_method_unchecked(
            main_class, parse_mid,
            ReturnType::Object,
            &[file_path_val.as_jni(), JValue::Int(max_length).as_jni()],
        )
    };
    jni_check_exception(&mut env)?; // prints any exceptions thrown to stderr
    let call_result_obj = call_result?.l()?;

    // Create and process the JStringResult
    let result = JStringResult::new(&mut env, call_result_obj)?;

    Ok(result.content)
}