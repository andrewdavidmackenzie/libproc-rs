#[cfg(target_os = "macos")]
fn main() {
    let bindings = bindgen::builder()
        .header_contents("libproc_rs.h", "#include <libproc.h>")
        .generate()
        .expect("Failed to build libproc bindings");

    bindings
        .write_to_file("src/osx_libproc_bindings.rs")
        .expect("Failed to write libproc bindings");
}

#[cfg(not(target_os = "macos"))]
fn main() {}
