use std::os::raw::{c_char, c_void};

use jni::{JavaVM, JNIEnv, sys};
use jni::errors::jni_error_code_to_result;
use jni::objects::{JString, JValue};

use crate::errors::{Error, ExtractResult};

/// Creates a new graalvm isolate using the invocation api. A [GraalVM isolate](https://medium.com/graalvm/isolates-and-compressed-references-more-flexible-and-efficient-memory-management-for-graalvm-a044cc50b67e) is a disjoint heap
/// that allows multiple tasks in the same VM instance to run independently.
///
/// This function uses the standard JVM invocation API and relies on the jni-sys crate.
/// No need to specify any libraries because the graalvm native image is already
/// linked in by the build script.
fn create_vm_isolate() -> ExtractResult<JavaVM> {
    unsafe {
        let mut options = sys::JavaVMOption {
            optionString: "-Djava.awt.headless=true".as_ptr() as *mut c_char,
            extraInfo: std::ptr::null_mut(),
        };
        let mut args = sys::JavaVMInitArgs {
            version: sys::JNI_VERSION_1_8,
            nOptions: 1,
            options: &mut options,
            ignoreUnrecognized: sys::JNI_TRUE,
        };
        let mut ptr: *mut sys::JavaVM = std::ptr::null_mut();
        let mut env: *mut sys::JNIEnv = std::ptr::null_mut();

        // The current thread becomes the main thread
        let jni_res =  sys::JNI_CreateJavaVM(
            &mut ptr as *mut _,
            &mut env as *mut *mut sys::JNIEnv as *mut *mut c_void,
            &mut args as *mut sys::JavaVMInitArgs as *mut c_void
        );
        jni_error_code_to_result(jni_res).map_err(|e| Error::JniError(e))?;

        let jvm = JavaVM::from_raw(ptr).map_err(|e| Error::JniError(e))?;

        Ok(jvm)
    }
}



fn cleanup_vm_isolate(jvm: &JavaVM, env: &mut JNIEnv) -> ExtractResult<()> {

    let exit_status = JValue::from(0);
    env.call_static_method("java/lang/System", "exit", "(I)V", &[exit_status])
        .map_err(|e| Error::JniError(e))?;

    unsafe {
        jvm.destroy().map_err(|e| Error::JniError(e))?;
    }

    Ok(())
}

/// Parse a file to a string using the Apache Tika library.
pub fn tika_parse_file(file_name: &str) -> ExtractResult<String> {
    let jvm = create_vm_isolate()?;
    let mut env = jvm.get_env().map_err(|e| Error::JniError(e))?;

    let jstr_file = env.new_string(file_name).map_err(|e| Error::JniError(e))?;

    let val = env.call_static_method("ai/yobix/TikaNativeMain", "parseToString",
                           "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::from(&jstr_file)])
        .map_err(|e| Error::JniError(e))?;

    let jobject = val.l().map_err(|e| Error::JniError(e))?;
    let jstr_output = JString::from(jobject);
    let javastr_output = env.get_string(&jstr_output).map_err(|e| Error::JniError(e))?;
    let output_str = javastr_output.to_str().map_err(|e| Error::Utf8Error(e))?;
    let output = output_str.to_string().clone();
    // Creates the string before cleaning the vm

    //cleanup_vm_isolate(&jvm, &mut env)?;

    Ok(output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_string_test() {
        let result = tika_parse_file("README.md");
        assert!(result.is_ok());
        assert_eq!("tika-native", result.unwrap());

    }
}