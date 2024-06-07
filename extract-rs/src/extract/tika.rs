use std::os::raw::{c_char, c_void};
use std::sync::OnceLock;

use jni::{AttachGuard, JavaVM, JNIEnv, sys};
use jni::errors::jni_error_code_to_result;
use jni::objects::{JObject, JString, JValue};
use jni::signature::ReturnType;

use crate::errors::{Error, ExtractResult};

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


/// Creates a new graalvm isolate using the invocation api. A [GraalVM isolate](https://medium.com/graalvm/isolates-and-compressed-references-more-flexible-and-efficient-memory-management-for-graalvm-a044cc50b67e) is a disjoint heap
/// that allows multiple tasks in the same VM instance to run independently.
///
/// This function uses the standard JVM invocation API and relies on the jni-sys crate.
/// No need to specify any libraries because the graalvm native image is already
/// linked in by the build script.
fn create_vm_isolate() -> JavaVM {
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
        let jni_res = sys::JNI_CreateJavaVM(
            &mut ptr as *mut _,
            &mut env as *mut *mut sys::JNIEnv as *mut *mut c_void,
            &mut args as *mut sys::JavaVMInitArgs as *mut c_void,
        );
        jni_error_code_to_result(jni_res).unwrap_or_else(|e| {
            panic!("Failed creating the graal native vm: {:?}", e);
        });

        // This sys call already attaches the current thread to the vm
        JavaVM::from_raw(ptr).unwrap_or_else(|e| {
            panic!("Failed creating the graal native from pointer: {:?}", e);
        })
    }
}

// fn cleanup_vm_isolate(jvm: JavaVM) -> ExtractResult<()>  {
//     println!("cleanup_vm_isolate");
//     // let mut env = jvm.attach_current_thread_as_daemon()?;
//     //
//     // let x = JValue::from(1);
//     // let system_class = env.find_class("java/lang/System")?;
//     // let exit_mid = env.get_static_method_id(&system_class, "exit", "(I)V")?;
//     // let _val = unsafe {
//     //     env.call_static_method_unchecked(
//     //         &system_class,
//     //         exit_mid,
//     //         ReturnType::Primitive(Primitive::Void),
//     //         &[x.as_jni()],
//     //     )
//     // };
//
//     // Destroy jvm. jvm must be dropped as well
//     unsafe {  jvm.destroy()?; }
//     drop(jvm);
//
//     Ok(())
// }

// pub fn tika_parse_file_new_vm(file_name: &str) -> ExtractResult<String> {
//
//     let mut output = String::new();
//
//     let mut start_time = Instant::now();
//     let jvm = create_vm_isolate();
//     let jvm_create_duration = start_time.elapsed();
//
//     start_time = Instant::now();
//     // Need to create a new scope to be able to drop intermediate objects before destroying the jvm
//     {
//         //let mut env = jvm.get_env()?;
//         let mut env = jvm.attach_current_thread()?;
//
//         let jstr_file = env.new_string(file_name)?;
//         let val = env.call_static_method("ai/yobix/TikaNativeMain", "parseToString",
//                                          "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::from(&jstr_file)])?;
//
//         let jobject = val.l()?;
//         let jstr_output = JString::from(jobject);
//         let javastr_output = env.get_string(&jstr_output)?;
//         let output_str = javastr_output.to_str().map_err(|e| Error::Utf8Error(e))?;
//         // Creates the string before cleaning the vm
//         output.push_str(output_str);
//     }
//     let parse_duration = start_time.elapsed();
//
//     start_time = Instant::now();
//     cleanup_vm_isolate(jvm)?;
//     let jvm_destroy_duration = start_time.elapsed();
//
//     println!("Time taken to jvm_create_duration: {:.4?}", jvm_create_duration);
//     println!("Time taken to parse_duration: {:.4?}", parse_duration);
//     println!("Time taken to jvm_destroy_duration: {:.4?}", jvm_destroy_duration);
//
//     Ok(output)
// }

/// Parses a file to a string using the Apache Tika library.
pub fn tika_parse_file(file_name: &str) -> ExtractResult<String> {

    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;

    let jstr_file = env.new_string(file_name)?;


    let main_class = env.find_class("ai/yobix/TikaNativeMain")?;
    let parse_mid = env.get_static_method_id(
        &main_class,
        "parseToString",
        "(Ljava/lang/String;)Lai/yobix/TikaResult;",
    )?;

    let parse_result = unsafe {
        env.call_static_method_unchecked(
            main_class, parse_mid,
            ReturnType::Object,
            &[JValue::from(&jstr_file).as_jni()],
        )
    };

    match parse_result {
        Ok(result) => {
            let jobject_result = result.l()?;

            let is_error = env.call_method(&jobject_result, "isError", "()Z", &[])?.z()?;
            if is_error {
                let jbyte_status = env.call_method(
                    &jobject_result, "getStatus", "()B", &[],
                )?.b()?;
                let jobject_error_msg = env.call_method(
                    &jobject_result, "getErrorMessage", "()Ljava/lang/String;", &[],
                )?.l()?;

                let error_msg = jobject_to_str(&mut env, jobject_error_msg)?;

                match jbyte_status {
                    1 => Err(Error::IoError(error_msg)),
                    2 => Err(Error::ParseError(error_msg)),
                    _ => Err(Error::Unknown(error_msg)),
                }
            } else {
                let jobject_output = env.call_method(
                    &jobject_result, "getContent", "()Ljava/lang/String;", &[],
                )?.l()?;

                let content = jobject_to_str(&mut env, jobject_output)?;
                Ok(content)
            }
        }
        Err(error) => {
            match error {
                jni::errors::Error::JavaException => {
                    let exception = jni_check_exception(&mut env)?;
                    match exception {
                        Some(message) => {
                            Err(Error::Unknown(message))
                        }
                        None => {
                            Err(Error::from(error))
                        }
                    }
                }
                _ => {
                    Err(Error::from(error))
                }
            }
        }
    }
}


fn jni_check_exception(env: &mut AttachGuard) -> ExtractResult<Option<String>> {

    if env.exception_check()? {
        env.exception_describe()?;
        env.exception_clear()?;
        return Ok(Some("Runtime exception occurred".to_string()))
    }

    Ok(None)

    // TODO parse exception into nice message
    //let exception_result = env.exception_occurred();
    // if exception_result.is_err() {
    //     return Ok(None);
    // }
    // let exception = exception_result.unwrap();
    //
    // if !exception.is_null() {
    //
    //     eprint!("exception.is_err() ");
    //     let jobject_ex = JObject::from(exception);
    //
    //     // Get the exception message
    //     let jobject_message = env.call_method(
    //         &jobject_ex, "getMessage", "()Ljava/lang/String;", &[],
    //     )?.l()?;
    //
    //     eprint!("exception.after() ");
    //
    //     let exc_message = if !jobject_message.is_null() {
    //         jobject_to_str(env, jobject_message)?
    //     } else {
    //         "".to_string()
    //     };
    //
    //
    //     // Get the exception cause message
    //     let jobject_cause = env.call_method(
    //         &jobject_ex, "getCause", "()Ljava/lang/Throwable;", &[],
    //     )?.l()?;
    //
    //     let exc_cause_message = if !jobject_cause.is_null() {
    //         let jobject_cause_message = env.call_method(
    //             &jobject_cause, "getMessage", "()Ljava/lang/String;", &[],
    //         )?.l()?;
    //
    //         if !jobject_cause_message.is_null() {
    //             jobject_to_str(env, jobject_cause_message)?
    //         } else {
    //             "".to_string()
    //         }
    //     } else {
    //         "".to_string()
    //     };
    //
    //     let output = format!("Exception: {} \n{}", exc_message, exc_cause_message);
    //
    //
    //     Ok(Some(output))
    // } else {
    //     Ok(None)
    // }
}

fn jobject_to_str(env: &mut JNIEnv, jobject: JObject) -> ExtractResult<String> {
    let jstring_output = JString::from(jobject);
    let javastr_output = unsafe { env.get_string_unchecked(&jstring_output)? };
    let output_str = javastr_output.to_str().map_err(Error::Utf8Error)?;

    Ok(output_str.to_string())
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