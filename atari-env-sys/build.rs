use std::path::PathBuf;

fn main() {
    // build & link ale_c
    let dst = {
        let mut make = cmake::Config::new("vendor/atari-py/atari_py/ale_interface/");

        #[cfg(feature = "sdl")]
        make.define("USE_SDL", "1");
        make.define("CMAKE_POLICY_VERSION_MINIMUM", "3.5");

        make.build()
    };

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=z");
    #[cfg(feature = "sdl")]
    println!("cargo:rustc-link-lib=dylib=SDL");
    println!("cargo:rustc-link-lib=static=ale_c");

    // gen bindings.rs
    let bindings = bindgen::Builder::default()
        .opaque_type("std::.*")
        .clang_arg(format!("-xc++"))
        .clang_arg(format!("-I{}/include/ale", dst.display()))
        .header("vendor/atari-py/atari_py/ale_c_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
