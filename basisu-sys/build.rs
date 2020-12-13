
use std::path::PathBuf;
use std::env;

fn main() {
    cc::Build::new()
		.cpp(true)
		.warnings(false)
        .file("vendor/transcoder/basisu_transcoder.cpp")
        .compile("libbasisu_transcoder.a");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
		.enable_cxx_namespaces()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.clang_arg("-x")
		.clang_arg("c++")
		.blacklist_item("FP_.*")
		.blacklist_type("size_type")
		.blacklist_type("std::size_type")
		.blacklist_type("std::collate_string_type")
		.blacklist_type("std::collate_byname_string_type")
		.blacklist_item("std::value")
		.blacklist_item("std::multiplier")
		.blacklist_item("std::increment")
		.blacklist_item("std::modulus")
		.blacklist_item("std::default_seed")
		.blacklist_item("std::xor_mask")
		.blacklist_item("std::tempering_.*")
		.blacklist_item("std::initialization_multiplier")
		.blacklist_item("__gnu_cxx::__min")
		.blacklist_item("__gnu_cxx::__max")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
