use std::io::Read;
use std::ops::{DerefMut};
use std::os::raw::{c_char, c_void};
use std::sync::OnceLock;
use bytemuck::cast_slice_mut;
use jni::{AttachGuard, JavaVM, JNIEnv, sys};
use jni::errors::jni_error_code_to_result;
use jni::objects::{JObject, JString, JValue};
use jni::signature::ReturnType;
use jni::sys::jsize;
use crate::errors::{Error, ExtractResult};

pub struct Reader<'a> {
    java_reader: JObject<'a>,
}

impl<'a> Drop for Reader<'a> {
    fn drop(&mut self) {
        match vm().attach_current_thread() {
            Ok(mut env) => {
                // Call the Java Reader's `close` method
                let _call_result = env.call_method
                (
                    &self.java_reader,
                    "close",
                    "()V",
                    &[],
                );
                // ignore result by using .ok()
                jni_check_exception(&mut env).ok();
            }
            Err(_) => { }
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
            &self.java_reader,
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

pub fn parse_file(file_name: &str) -> ExtractResult<Reader> {
    let mut env = vm().attach_current_thread()?;

    let jstr_file = env.new_string(file_name)?;

    let main_class = env.find_class("ai/yobix/TikaNativeMain")?;
    let parse_mid = env.get_static_method_id(
        &main_class,
        "parse",
        "(Ljava/lang/String;)Lai/yobix/ReaderResult;",
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

                let error_msg = jobject_to_str(env.deref_mut(), jobject_error_msg)?;

                match jbyte_status {
                    1 => Err(Error::IoError(error_msg)),
                    _ => Err(Error::Unknown(error_msg)),
                }
            } else {

                let jobject_output_result = env.call_method(
                    &jobject_result, "getReader", "()Lorg/apache/commons/io/input/ReaderInputStream;", &[],
                );
                jni_check_exception(&mut env)?;
                let jobject_output = jobject_output_result?.l()?;
                let reader = Reader { java_reader: jobject_output };

                Ok(reader)
            }
        }
        Err(error) => {
            jni_check_exception(&mut env)?;
            Err(Error::from(error))
        }
    }
}

/// Parses a file to a string using the Apache Tika library.
pub fn parse_file_to_string(file_name: &str) -> ExtractResult<String> {

    // Attaching a thead that is already attached is a no-op. Good to have this in case this method
    // is called from another thread
    let mut env = vm().attach_current_thread()?;

    let jstr_file = env.new_string(file_name)?;


    let main_class = env.find_class("ai/yobix/TikaNativeMain")?;
    let parse_mid = env.get_static_method_id(
        &main_class,
        "parseToString",
        "(Ljava/lang/String;)Lai/yobix/StringResult;",
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

            jni_check_string_result(&mut env, jobject_result)
        }
        Err(error) => {
            jni_check_exception(&mut env)?;
            Err(Error::from(error))
        }
    }
}

fn jni_check_string_result(env: &mut AttachGuard, jobject_result: JObject) -> ExtractResult<String> {
    let is_error = env.call_method(&jobject_result, "isError", "()Z", &[])?.z()?;
    if is_error {
        let jbyte_status = env.call_method(
            &jobject_result, "getStatus", "()B", &[],
        )?.b()?;
        let jobject_error_msg = env.call_method(
            &jobject_result, "getErrorMessage", "()Ljava/lang/String;", &[],
        )?.l()?;

        let error_msg = jobject_to_str(env, jobject_error_msg)?;

        match jbyte_status {
            1 => Err(Error::IoError(error_msg)),
            2 => Err(Error::ParseError(error_msg)),
            _ => Err(Error::Unknown(error_msg)),
        }
    } else {
        let jobject_output = env.call_method(
            &jobject_result, "getContent", "()Ljava/lang/String;", &[],
        )?.l()?;

        let content = jobject_to_str(env, jobject_output)?;
        Ok(content)
    }
}

fn jni_check_exception(env: &mut AttachGuard) -> ExtractResult<bool> {
    if env.exception_check()? {
        env.exception_describe()?;
        env.exception_clear()?;
        return Ok(true)
    }
    Ok(false)
}

fn jobject_to_str(env: &mut JNIEnv, jobject: JObject) -> ExtractResult<String> {
    let jstring_output = JString::from(jobject);
    let javastr_output = unsafe { env.get_string_unchecked(&jstring_output)? };
    let output_str = javastr_output.to_str().map_err(Error::Utf8Error)?;

    Ok(output_str.to_string())
}


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
        // let mut option0 = sys::JavaVMOption {
        //     optionString: "-Djava.awt.headless=true".as_ptr() as *mut c_char,
        //     extraInfo: std::ptr::null_mut(),
        // };

        // Set java.library.path to be able to load libawt.so, which must be in the same dir as libtika_native.so
        let mut options = sys::JavaVMOption {
            optionString: "-Djava.library.path=.".as_ptr() as *mut c_char,
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