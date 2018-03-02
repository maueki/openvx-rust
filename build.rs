extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rustc-link-lib=openvx");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib");

    let bindings = bindgen::Builder::default()
        .header("/opt/rocm/include/VX/vx.h")
        .clang_arg("-I/opt/rocm/include")
        .blacklist_type("vx_pixel_value_t")
        .blacklist_type("_vx_pixel_value_t")
        .constified_enum_module("vx_kernel_e")
        .constified_enum_module("vx_df_image_e")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
