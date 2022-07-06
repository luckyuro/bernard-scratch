use dart_bindgen::{config::*, Codegen};

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config {
        language: cbindgen::Language::C,
        ..Default::default()
    };
    config.braces = cbindgen::Braces::SameLine;
    config.cpp_compat = true;
    config.style = cbindgen::Style::Both;
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("binding.h");

    let config = DynamicLibraryConfig {
        ios: DynamicLibraryCreationMode::Executable.into(),
        android: DynamicLibraryCreationMode::open("libscrap_ffi.so").into(),
        linux: DynamicLibraryCreationMode::Executable.into(),
        macos: DynamicLibraryCreationMode::Executable.into(),
        windows: DynamicLibraryCreationMode::Executable.into(),
        ..Default::default()
    };

    let codegen = Codegen::builder()
        .with_src_header("binding.h")
        .with_lib_name("libscrap")
        .with_config(config)
        .with_allo_isolate()
        .build()
        .unwrap();

    let bindings = codegen.generate().unwrap();    


    bindings
        .write_to_file("../../packages/scrap_ffi/lib/ffi.dart")
        .unwrap();
}
