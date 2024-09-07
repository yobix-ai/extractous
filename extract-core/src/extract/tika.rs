use std::io::Read;
use std::sync::OnceLock;

use bytemuck::cast_slice_mut;
use jni::JavaVM;
use jni::objects::{JObject, JValue};
use jni::signature::ReturnType;
use jni::sys::jsize;

use crate::errors::ExtractResult;
use crate::extract::jni_utils::*;
use crate::extract::wrappers::*;
use crate::PdfParserConfig;

/// Returns a reference to the shared VM isolate
/// Instead of creating a new VM for every tika call, we create a single VM that is shared
/// throughout the application.
fn vm() -> &'static JavaVM {
    // static items do not call `Drop` on program termination
    static GRAAL_VM: OnceLock<JavaVM> = OnceLock::new();
    GRAAL_VM.get_or_init(|| {
        create_vm_isolate()
    })
}


pub struct Reader<'a> {
    internal: JObject<'a>,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(obj: JObject<'a>) -> Self {
        Self {
            internal: obj,
        }
    }
}

impl<'a> Drop for Reader<'a> {
    fn drop(&mut self) {
        match vm().attach_current_thread() {
            Ok(mut env) => {
                // Call the Java Reader's `close` method
                let _call_result = env.call_method
                (
                    &self.internal,
                    "close",
                    "()V",
                    &[],
                );
                // ignore result by using .ok()
                jni_check_exception(&mut env).ok();
            }
            Err(_) => {}
        }
    }
}

impl<'a> Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut env = vm().attach_current_thread().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to attach thread: {:?}", e))
        })?;

        let length = buf.len() as jsize;
        let jbyte_array = env.new_byte_array(length).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create byte array: {:?}", e))
        })?;

        // Call the Java Reader's `read` method
        let call_result = env.call_method
        (
            &self.internal,
            "read",
            "([BII)I",
            &[JValue::Object(&jbyte_array), JValue::Int(0), JValue::Int(length)],
        );
        // Check for any java exception thrown, prints to stderr and ignore the result
        jni_check_exception(&mut env).ok();

        let result = call_result.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to call read method: {:?}", e))
        })?.i().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to unwrap result to jint: {:?}", e))
        })?;

        let buf_of_i8: &mut [i8] = cast_slice_mut(buf);
        env.get_byte_array_region(jbyte_array, 0, buf_of_i8).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get char array region: {:?}", e))
        })?;

        if result == -1 {
            // End of stream reached
            Ok(0)
        } else {
            Ok(result as usize)
        }
    }
}

pub fn parse_file<'local>(
    file_path: &str,
    pdf_conf: &'local PdfParserConfig
) -> ExtractResult<Reader<'local>> {
    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;

    let file_path_val = jni_new_string_as_jvalue(&mut env, file_path)?;

    let jpdf_conf = JPDFParserConfig::new(&mut env, pdf_conf)?;

    // Make the parse call
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

    Ok(Reader::new(result.java_reader))
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