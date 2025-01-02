use std::{env, fs};
use std::path::PathBuf;

fn main() {
    let dovecot_dir = fs::canonicalize(env::current_dir().unwrap().join("dovecot")).unwrap();
    let src_dir = dovecot_dir.join("src");
    let lib_dir = src_dir.join("lib");
    let lib_storage_dir = src_dir.join("lib-storage");
    let lib_mail_dir = src_dir.join("lib-mail");
    let lib_index_dir = src_dir.join("lib-index");
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("--include-directory={}", dovecot_dir.display()))
        .clang_arg(format!("--include-directory={}", src_dir.display()))
        .clang_arg(format!("--include-directory={}", lib_storage_dir.display()))
        .clang_arg(format!("--include-directory={}", lib_mail_dir.display()))
        .clang_arg(format!("--include-directory={}", lib_index_dir.display()))
        .clang_arg(format!("--include-directory={}", lib_dir.display()))
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}