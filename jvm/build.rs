use std::path::Path;
use rust_swig::{LanguageConfig, JavaConfig};
use std::env;

fn main() {
    println!(r"cargo:rustc-link-search={}", env!("CARGO_MANIFEST_DIR"));
    let out_dir = env::var("OUT_DIR").unwrap();

    let in_src = Path::new("src");
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    let swig_gen = rust_swig::Generator::new(LanguageConfig::JavaConfig(JavaConfig::new(
        Path::new("java").join("com").join("example").join("rust"),
        "com.example.rust".into(),
    )));
    swig_gen.expand("rust_swig_opa_jni", &in_src, &out_src);
}