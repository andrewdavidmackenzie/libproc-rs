fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "macos" {
        build_macos_bindings();
    }
}

fn build_macos_bindings() {
    use bindgen::{RustEdition, RustTarget};
    use std::env;
    use std::path::Path;

    match RustTarget::stable(72, 0) {
        Ok(rust_target) => {
            let sdk_path = env::var("SDKROOT")
                .unwrap_or_else(|_| "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk".into());

            let bindings = bindgen::builder()
                .header_contents("libproc_rs.h", "#include <libproc.h>")
                .rust_target(rust_target)
                .rust_edition(RustEdition::Edition2018)
                .layout_tests(false)
                .clang_args(&["-x", "c++", "-I", &format!("{sdk_path}/usr/include/")])
                .generate()
                .expect("Failed to build libproc bindings");

            let output_path =
                Path::new(&env::var("OUT_DIR").expect("OUT_DIR env var was not defined"))
                    .join("osx_libproc_bindings.rs");

            bindings
                .write_to_file(output_path)
                .expect("Failed to write libproc bindings");
        }
        _ => eprintln!("Error executing bindgen"),
    }
}
