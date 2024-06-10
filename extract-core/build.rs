use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    let tika_native_dir = root_dir.join("tika-native");
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let profile = env::var("PROFILE").unwrap();
    let dist_dir = root_dir.join("target").join(profile).join("deps");

    // Rerun this build script if the tika-native build directory changes
    //let tika_build_path = root_dir.join("tika-native/build/native/nativeCompile");
    //println!("cargo::rerun-if-changed={}", tika_build_path.display());

    // Check that graalvm and gradle are installed
    check_graalvm(&target_os);

    // Just for debugging
    //println!("cargo:warning=gradlew: {}", gradlew);
    //println!("cargo:warning=dist_dir: {}", dist_dir.display());
    // println!("cargo:warning=out_dir: {}", out_dir.display());


    gradle_build(&target_os, &tika_native_dir, &out_dir, &dist_dir);

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", out_dir.display());
    //println!("cargo:rustc-link-search={}", dist_dir.display());
    // Tell cargo to tell rustc to link the `tika_native` shared library.
    println!("cargo:rustc-link-lib=dylib=tika_native");
}

// Run the gradle build command to build tika-native
fn gradle_build(target_os: &str, tika_native_dir: &PathBuf,
                out_dir: &PathBuf, _dist_dir: &Path
) {
    let gradlew = match target_os {
        "windows" => "./gradlew.bat",
        _ => "./gradlew"
    };

    Command::new(gradlew)
        .current_dir(tika_native_dir)
        .arg("nativeCompile")
        .status()
        .expect("Failed to build tika-native");

    let build_path = tika_native_dir.join("build/native/nativeCompile");

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    fs_extra::dir::copy(build_path, out_dir, &options)
         .expect("Failed to copy build artifacts to OUTPUT_DIR");
    //fs_extra::dir::copy(&build_path, dist_dir, &options)
    //    .expect("Failed to copy build artifacts to DIST_DIR");
}

// checks if GraalVM JDK is installed and pointed to by JAVA_HOME or panics if it can't be found
pub fn check_graalvm(target_os: &str) {
    let graalvm_version = match target_os {
        "macos" => "24.0.1.r22-nik",
        _ => "22.0.1-graalce"
    };
    let help_msg = format!("\nWe recommend using sdkman to install and \
                manage different JDKs. See https://sdkman.io/usage for more information.\n\
                You can install graalvm using:\n  \
                sdk install java {} \n  \
                sdk use java {}", graalvm_version, graalvm_version);

    let java_home = env::var_os("JAVA_HOME").map(PathBuf::from);

    match java_home {
        Some(java_home) => {
            // Check that native-image is in JAVA_HOME/bin
            let native_image = java_home.join("bin").join("native-image");
            if !native_image.exists() {
                panic!("Your JAVA_HOME env variable is pointing to: {}. Please make sure your \
                JAVA_HOME is pointing to a valid GraalVM JDK. {}", java_home.display(), help_msg);
            }
        }
        None => {
            panic!("Could not find a valid GraalVM JDK. Please make sure your your JAVA_HOME is
            to pointing to a valid GraalVM JDK. {}", help_msg);
        }
    }
}