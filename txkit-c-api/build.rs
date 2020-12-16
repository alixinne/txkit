fn main() {
    std::fs::write(
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Failed to get OUT_DIR"))
            .join("config.rs"),
        format!(
            r#"pub const OUT_DIR: &'static str = "{}";
    pub const TARGET: &'static str = "{}";
    pub const OPT_LEVEL: &'static str = "{}";
    pub const HOST: &'static str = "{}";
    "#,
            std::env::var("OUT_DIR").unwrap(),
            std::env::var("TARGET").unwrap(),
            std::env::var("OPT_LEVEL").unwrap(),
            std::env::var("HOST").unwrap()
        ),
    )
    .unwrap();
}
