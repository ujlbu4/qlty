fn main() {
    if std::env::var("CARGO_FEATURE_QLTY_LLM").is_ok() {
        println!("cargo:rustc-cfg=feature=\"llm\"");
    }
    if std::env::var("CARGO_FEATURE_QLTY_SKILLS").is_ok() {
        println!("cargo:rustc-cfg=feature=\"skills\"");
    }
}
