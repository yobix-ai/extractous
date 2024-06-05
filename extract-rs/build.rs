use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    let tika_native_dir = root_dir.join("tika-native");
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let target = env::var("TARGET").unwrap();

    let profile = env::var("PROFILE").unwrap();
    let dist_dir = root_dir.join("target").join(profile).join("deps");

    // Rerun this build script if the tika-native build directory changes
    //let tika_build_path = root_dir.join("tika-native/build/native/nativeCompile");
    //println!("cargo::rerun-if-changed={}", tika_build_path.display());

    // Check that graalvm and gradle are installed
    let _graalvm_home = get_graalvm(&target);
    let gradle = get_gradle(&target);

    // Just for debugging
    // println!("cargo:warning=dist_dir: {}", dist_dir.display());
    // println!("cargo:warning=graalvm_home: {}", _graalvm_home.display());
    // println!("cargo:warning=gradle: {}", gradle.display());
    // println!("cargo:warning=target: {}", target);
    // println!("cargo:warning=out_dir: {}", out_dir.display());

    // Build tika-native
    // if !tika_build_path.exists() {
    //     println!("cargo:warning=Building tika-native, this may take a while. Please be patient ...");
    // }
    gradle_build(&gradle, &tika_native_dir, &out_dir, &dist_dir);

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-search={}", dist_dir.display());
    // Tell cargo to tell rustc to link the `tika_native` shared library.
    println!("cargo:rustc-link-lib=dylib=tika_native");
}

// Run the gradle build command to build tika-native
fn gradle_build(gradle: &PathBuf, tika_native_dir: &PathBuf,
                out_dir: &PathBuf, dist_dir: &PathBuf
) {
    Command::new(gradle)
        .current_dir(tika_native_dir)
        .arg("nativeCompile")
        .status()
        .expect("Failed to build tika-native");

    let build_path = tika_native_dir.join("build/native/nativeCompile");

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    fs_extra::dir::copy(&build_path, out_dir, &options)
        .expect("Failed to copy build artifacts to OUTPUT_DIR");
    fs_extra::dir::copy(&build_path, dist_dir, &options)
        .expect("Failed to copy build artifacts to OUTPUT_DIR");
}

// Return the path to the GraalVM JDK or panics if it can't be found
pub fn get_graalvm(_target: &str) -> PathBuf {
    let help_msg = "\nWe recommend using sdkman to install and \
                manage different JDKs. See https://sdkman.io/usage for more information.\n\
                You can install graalvm using:\n  \
                sdk install java 22.0.1-graalce \n  \
                sdk use java  22.0.1-graalce";

    let java_home = env::var_os("JAVA_HOME").map(PathBuf::from);

    match java_home {
        Some(java_home) => {
            // Check that native-image is in JAVA_HOME/bin
            let native_image = java_home.join("bin").join("native-image");
            if native_image.exists() {
                java_home
            } else {
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

// Return the path to Gradle or panics if it can't be found
pub fn get_gradle(_target: &str) -> PathBuf {
    let help_msg = "\nWe recommend using sdkman to install and \
                manage different gradle installations. See https://sdkman.io/usage for more \
                information.\n\
                You can install gradle using:\n  \
                sdk install gradle 8.7 \n  \
                sdk use gradle 8.7";

    let gradle_home = env::var_os("GRADLE_HOME").map(PathBuf::from);

    match gradle_home {
        Some(gradle_home) => {
            // Check that native-image is in JAVA_HOME/bin
            let gradle = gradle_home.join("bin").join("gradle");
            if gradle.exists() {
                gradle
            } else {
                panic!("Your GRADLE_HOME env variable is pointing to: {}. Please make sure your \
                GRADLE_HOME is pointing to a valid GRADLE installation. {}", gradle_home.display
                (), help_msg);
            }
        }
        None => {
            panic!("Could not find a valid GRADLE installation. Please make sure your your
            GRADLE_HOME is to pointing to a valid GRADLE installation. {}", help_msg);
        }
    }
}