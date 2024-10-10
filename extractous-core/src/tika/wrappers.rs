use crate::errors::{Error, ExtractResult};
use crate::tika::jni_utils::{jni_call_method, jni_jobject_array_to_vec, jni_jobject_to_string, jni_new_string_as_jvalue};
use crate::tika::vm;
use crate::{DEFAULT_BUF_SIZE, OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};
use bytemuck::cast_slice_mut;
use jni::objects::{GlobalRef, JByteArray, JObject, JValue};
use jni::sys::jsize;
use jni::JNIEnv;

/// Wrapper for [`JObject`]s that contain `org.apache.commons.io.input.ReaderInputStream`
/// It saves a GlobalRef to the java object, which is cleared when the last GlobalRef is dropped
/// Implements [`Drop] trait to properly close the `org.apache.commons.io.input.ReaderInputStream`
#[derive(Clone)]
pub struct JReaderInputStream {
    internal: GlobalRef,
    buffer: GlobalRef,
    capacity: jsize,
}

impl JReaderInputStream {
    pub(crate) fn new<'local>(
        env: &mut JNIEnv<'local>,
        obj: JObject<'local>,
    ) -> ExtractResult<Self> {
        // Creates new jbyte array
        let capacity = DEFAULT_BUF_SIZE as jsize;
        let jbyte_array = env.new_byte_array(capacity)?;

        Ok(Self {
            internal: env.new_global_ref(obj)?,
            buffer: env.new_global_ref(jbyte_array)?,
            capacity,
        })
    }

    pub(crate) fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut env = vm().attach_current_thread().map_err(Error::JniError)?;

        let length = buf.len() as jsize;

        if length > self.capacity {
            // Create the new byte array with the new capacity
            let jbyte_array = env
                .new_byte_array(length as jsize)
                .map_err(|_e| Error::JniEnvCall("Failed to create byte array"))?;

            self.buffer = env
                .new_global_ref(jbyte_array)
                .map_err(|_e| Error::JniEnvCall("Failed to create global reference"))?;

            self.capacity = length;
        }

        // // Create the java byte array
        // let length = buf.len() as jsize;
        // let jbyte_array = env
        //     .new_byte_array(length)
        //     .map_err(|_e| Error::JniEnvCall("Failed to create byte array"))?;

        // Call the Java Reader's `read` method
        let call_result = jni_call_method(
            &mut env,
            &self.internal,
            "read",
            "([BII)I",
            &[
                JValue::Object(&self.buffer),
                JValue::Int(0),
                JValue::Int(length),
            ],
        );
        let num_read_bytes = call_result?.i().map_err(Error::JniError)?;

        // Get self.buffer object as a local reference
        let obj_local = env
            .new_local_ref(&self.buffer)
            .map_err(|_e| Error::JniEnvCall("Failed to create local ref"))?;

        // cast because java byte array is i8[]
        let buf_of_i8: &mut [i8] = cast_slice_mut(buf);

        // Get the bytes from the Java byte array to the Rust byte array
        // This is a copy or just memory reference. POTENTIAL performance improvement
        env.get_byte_array_region(JByteArray::from(obj_local), 0, buf_of_i8)
            .map_err(|_e| Error::JniEnvCall("Failed to get byte array region"))?;

        if num_read_bytes == -1 {
            // End of stream reached
            Ok(0)
        } else {
            Ok(num_read_bytes as usize)
        }
    }
}

impl Drop for JReaderInputStream {
    fn drop(&mut self) {
        if let Ok(mut env) = vm().attach_current_thread() {
            // Call the Java Reader's `close` method
            jni_call_method(&mut env, &self.internal, "close", "()V", &[]).ok();
        }
    }
}

/// Wrapper for the Java class  `ai.yobix.StringResult`
/// Upon creation it parses the java StringResult object and saves the converted Rust string
pub struct JResult {
    pub content: String,
    pub metadata: Vec<String>,
}

impl<'local> JResult {
    pub(crate) fn new(env: &mut JNIEnv<'local>, obj: JObject<'local>) -> ExtractResult<Self> {
        let is_error = jni_call_method(env, &obj, "isError", "()Z", &[])?.z()?;

        if is_error {
            let status = jni_call_method(env, &obj, "getStatus", "()B", &[])?.b()?;
            let msg_obj = env
                .call_method(&obj, "getErrorMessage", "()Ljava/lang/String;", &[])?
                .l()?;
            let msg = jni_jobject_to_string(env, msg_obj)?;
            match status {
                1 => Err(Error::IoError(msg)),
                2 => Err(Error::ParseError(msg)),
                _ => Err(Error::Unknown(msg)),
            }
        } else {
            let call_result_obj = env
                .call_method(&obj, "getContent", "()Ljava/lang/String;", &[])?
                .l()?;
            let content = jni_jobject_to_string(env, call_result_obj)?;

            let metadata_obj_array = env
                .call_method(&obj, "getMetadata", "()[Ljava/lang/String;", &[])?
                .l()?;
            let metadata = jni_jobject_array_to_vec(env, metadata_obj_array)?;

            Ok(Self { content, metadata })
        }
    }
}

/// Wrapper for the Java class  `ai.yobix.ReaderResult`
/// Upon creation it parses the java ReaderResult object and saves the java
/// `org.apache.commons.io.input.ReaderInputStream` object, which later can be used for reading
pub(crate) struct JReaderResult<'local> {
    pub(crate) java_reader: JObject<'local>,
}

impl<'local> JReaderResult<'local> {
    pub(crate) fn new(env: &mut JNIEnv<'local>, obj: JObject<'local>) -> ExtractResult<Self> {
        let is_error = jni_call_method(env, &obj, "isError", "()Z", &[])?.z()?;

        if is_error {
            let status = jni_call_method(env, &obj, "getStatus", "()B", &[])?.b()?;
            let msg_obj = env
                .call_method(&obj, "getErrorMessage", "()Ljava/lang/String;", &[])?
                .l()?;
            let msg = jni_jobject_to_string(env, msg_obj)?;
            match status {
                1 => Err(Error::IoError(msg)),
                2 => Err(Error::ParseError(msg)),
                _ => Err(Error::Unknown(msg)),
            }
        } else {
            let reader_obj = jni_call_method(
                env,
                &obj,
                "getReader",
                "()Lorg/apache/commons/io/input/ReaderInputStream;",
                &[],
            )?
                .l()?;

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
        // Create the java object
        let class = env.find_class("org/apache/tika/parser/pdf/PDFParserConfig")?;
        let obj = env.new_object(&class, "()V", &[])?;

        // Call the setters
        // Make sure all of these methods are declared in jni-config.json file, otherwise
        // java method not found exception will be thrown
        jni_call_method(
            env,
            &obj,
            "setExtractInlineImages",
            "(Z)V",
            &[JValue::from(config.extract_inline_images)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setExtractUniqueInlineImagesOnly",
            "(Z)V",
            &[JValue::from(config.extract_unique_inline_images_only)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setExtractMarkedContent",
            "(Z)V",
            &[JValue::from(config.extract_marked_content)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setExtractAnnotationText",
            "(Z)V",
            &[JValue::from(config.extract_annotation_text)],
        )?;
        // The PdfOcrStrategy enum names must match the Java org.apache.tika.parser.pdf
        // .PDFParserConfig$OCR_STRATEGY enum names
        let ocr_str_val = jni_new_string_as_jvalue(env, &config.ocr_strategy.to_string())?;
        jni_call_method(
            env,
            &obj,
            "setOcrStrategy",
            "(Ljava/lang/String;)V",
            &[(&ocr_str_val).into()],
        )?;

        Ok(Self { internal: obj })
    }
}

/// Wrapper for [`JObject`]s that contain `org.apache.tika.parser.microsoft.OfficeParserConfig`.
pub(crate) struct JOfficeParserConfig<'local> {
    pub(crate) internal: JObject<'local>,
}

impl<'local> JOfficeParserConfig<'local> {
    /// Creates a new object instance of `JOfficeParserConfig` in the java world
    /// keeps reference to the object for later use
    pub(crate) fn new(
        env: &mut JNIEnv<'local>,
        config: &OfficeParserConfig,
    ) -> ExtractResult<Self> {
        // Create the java object
        let class = env.find_class("org/apache/tika/parser/microsoft/OfficeParserConfig")?;
        let obj = env.new_object(&class, "()V", &[])?;

        // Call the setters
        // Make sure all of these methods are declared in jni-config.json file, otherwise
        // java method not found exception will be thrown
        jni_call_method(
            env,
            &obj,
            "setExtractMacros",
            "(Z)V",
            &[JValue::from(config.extract_macros)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeDeletedContent",
            "(Z)V",
            &[JValue::from(config.include_deleted_content)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeMoveFromContent",
            "(Z)V",
            &[JValue::from(config.include_move_from_content)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeShapeBasedContent",
            "(Z)V",
            &[JValue::from(config.include_shape_based_content)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeHeadersAndFooters",
            "(Z)V",
            &[JValue::from(config.include_headers_and_footers)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeMissingRows",
            "(Z)V",
            &[JValue::from(config.include_missing_rows)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeSlideNotes",
            "(Z)V",
            &[JValue::from(config.include_slide_notes)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setIncludeSlideMasterContent",
            "(Z)V",
            &[JValue::from(config.include_slide_master_content)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setConcatenatePhoneticRuns",
            "(Z)V",
            &[JValue::from(config.concatenate_phonetic_runs)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setExtractAllAlternativesFromMSG",
            "(Z)V",
            &[JValue::from(config.extract_all_alternatives_from_msg)],
        )?;

        Ok(Self { internal: obj })
    }
}

/// Wrapper for [`JObject`]s that contain `org.apache.tika.parser.ocr.TesseractOCRConfig`.
pub(crate) struct JTesseractOcrConfig<'local> {
    pub(crate) internal: JObject<'local>,
}
impl<'local> JTesseractOcrConfig<'local> {
    /// Creates a new object instance of `JTesseractOcrConfig` in the java world
    /// keeps reference to the object for later use
    pub(crate) fn new(
        env: &mut JNIEnv<'local>,
        config: &TesseractOcrConfig,
    ) -> ExtractResult<Self> {
        // Create the java object
        let class = env.find_class("org/apache/tika/parser/ocr/TesseractOCRConfig")?;
        let obj = env.new_object(&class, "()V", &[])?;

        // Call the setters
        // Make sure all of these methods are declared in jni-config.json file, otherwise
        // java method not found exception will be thrown
        jni_call_method(
            env,
            &obj,
            "setDensity",
            "(I)V",
            &[JValue::from(config.density)],
        )?;
        jni_call_method(env, &obj, "setDepth", "(I)V", &[JValue::from(config.depth)])?;
        jni_call_method(
            env,
            &obj,
            "setTimeoutSeconds",
            "(I)V",
            &[JValue::from(config.timeout_seconds)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setEnableImagePreprocessing",
            "(Z)V",
            &[JValue::from(config.enable_image_preprocessing)],
        )?;
        jni_call_method(
            env,
            &obj,
            "setApplyRotation",
            "(Z)V",
            &[JValue::from(config.apply_rotation)],
        )?;

        let lang_string_val = jni_new_string_as_jvalue(env, &config.language)?;
        jni_call_method(
            env,
            &obj,
            "setLanguage",
            "(Ljava/lang/String;)V",
            &[(&lang_string_val).into()],
        )?;

        Ok(Self { internal: obj })
    }
}
