use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::signature::{Primitive, ReturnType};

use crate::errors::{Error, ExtractResult};
use crate::extract::jni_utils::jni_jobject_to_string;
use crate::PdfParserConfig;

pub(crate) struct JStringResult {
    pub(crate) content: String,
}

impl<'local> JStringResult {

    pub(crate) fn new(env: &mut JNIEnv<'local>, obj: JObject<'local>) -> ExtractResult<Self> {

        let is_error = env.call_method(&obj, "isError", "()Z", &[])?.z()?;

        if is_error {
            let status = env.call_method(
                &obj, "getStatus", "()B", &[]
            )?.b()?;
            let msg_obj = env.call_method(
                &obj, "getErrorMessage", "()Ljava/lang/String;", &[],
            )?.l()?;
            let msg = jni_jobject_to_string(env, msg_obj)?;
            match status {
                1 => Err(Error::IoError(msg)),
                2 => Err(Error::ParseError(msg)),
                _ => Err(Error::Unknown(msg)),
            }
        } else {

            let call_result_obj = env.call_method(
                &obj, "getContent", "()Ljava/lang/String;", &[],
            )?.l()?;

            let content = jni_jobject_to_string(env, call_result_obj)?;

            Ok(Self {
                content
            })
        }
    }
}


pub(crate) struct JReaderResult<'local> {
    pub(crate) java_reader: JObject<'local>,
}

impl<'local> JReaderResult<'local> {
    pub(crate) fn new(env: &mut JNIEnv<'local>, obj: JObject<'local>) -> ExtractResult<Self> {

        let is_error = env.call_method(&obj, "isError", "()Z", &[])?.z()?;

        if is_error {
            let status = env.call_method(
                &obj, "getStatus", "()B", &[]
            )?.b()?;
            let msg_obj = env.call_method(
                &obj, "getErrorMessage", "()Ljava/lang/String;", &[],
            )?.l()?;
            let msg = jni_jobject_to_string(env, msg_obj)?;
            match status {
                1 => Err(Error::IoError(msg)),
                2 => Err(Error::ParseError(msg)),
                _ => Err(Error::Unknown(msg)),
            }
        } else {

            let reader_obj = env.call_method(
                &obj, "getReader", "()Lorg/apache/commons/io/input/ReaderInputStream;", &[],
            )?.l()?;

            Ok(Self {
                java_reader: reader_obj,
            })
        }
    }
}


/// Wrapper for [`JObject`]s that contain `org.apache.tika.parser.pdf.PDFParserConfig`.
/// Looks up the class and method IDs on creation rather than for every method call.
pub(crate) struct JPDFParserConfig<'local> {
    pub(crate) internal: JObject<'local>,
}

impl<'local> JPDFParserConfig<'local> {
    /// Creates a new object instance of `JPDFParserConfig` in the java world
    /// keeps reference to the object and method IDs for later use
    pub(crate) fn new(env: &mut JNIEnv<'local>, config: &PdfParserConfig) -> ExtractResult<Self> {
        let class = env.find_class("org/apache/tika/parser/pdf/PDFParserConfig")?;

        let mid_set_extract_inline_images =
            env.get_method_id(&class, "setExtractInlineImages", "(Z)V")?;

        let mid_set_extract_marked_content =
            env.get_method_id(&class, "setExtractMarkedContent", "(Z)V")?;

        // Create the java object
        let obj = env.new_object(&class, "()V", &[])?;

        // Set all teh fields, we use unchecked calls because it's faster
        unsafe {
            env.call_method_unchecked(
                &obj,
                mid_set_extract_inline_images,
                ReturnType::Primitive(Primitive::Void),
                &[JValue::from(config.extract_inline_images).as_jni()],
            )?;
            env.call_method_unchecked(
                &obj,
                mid_set_extract_marked_content,
                ReturnType::Primitive(Primitive::Void),
                &[JValue::from(config.extract_marked_content).as_jni()],
            )?;
        };

        Ok(Self {
            internal: obj,
        })
    }
}