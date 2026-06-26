fn main() {
    // On Windows, tell the linker where to find mpv.lib
    // MPV_LIB_DIR is set by the GitHub Actions workflow
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        if let Ok(lib_dir) = std::env::var("MPV_LIB_DIR") {
            println!("cargo:rustc-link-search=native={}", lib_dir);
        }
        println!("cargo:rustc-link-lib=mpv");
    }

    tauri_build::build()
}
