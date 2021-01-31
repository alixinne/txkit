use quote::quote;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target = std::env::var("TARGET").unwrap();
    let opt_level = std::env::var("OPT_LEVEL").unwrap();
    let host = std::env::var("HOST").unwrap();

    let module = quote! {
        pub const OUT_DIR: &'static str = #out_dir;
        pub const TARGET: &'static str = #target;
        pub const OPT_LEVEL: &'static str = #opt_level;
        pub const HOST: &'static str = #host;
    };

    std::fs::write(
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Failed to get OUT_DIR"))
            .join("config.rs"),
        module.to_string(),
    )
    .unwrap();
}
