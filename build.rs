use std::env;
use std::path::PathBuf;

#[cfg(feature = "gpu")]
fn wrap_shaders() {
    let mut compiler = tinygl_compiler::CompilerBuilder::default().build().unwrap();

    let debug_frag = compiler.wrap_shader("shaders/debug.frag").unwrap();
    let quad_vert = compiler.wrap_shader("shaders/quad.vert").unwrap();
    let whitenoise_frag = compiler.wrap_shader("shaders/whitenoise.frag").unwrap();

    let debug_prog = compiler
        .wrap_program(&[&quad_vert, &debug_frag], "debug")
        .unwrap();

    let whitenoise_prog = compiler
        .wrap_program(&[&quad_vert, &whitenoise_frag], "whitenoise")
        .unwrap();

    let global_set = compiler
        .wrap_uniforms(&[&debug_prog, &whitenoise_prog], "global")
        .unwrap();

    compiler
        .write_root_include(
            env::var("OUT_DIR").unwrap(),
            &[
                &debug_frag,
                &quad_vert,
                &whitenoise_frag,
                &debug_prog,
                &whitenoise_prog,
                &global_set,
            ],
        )
        .unwrap();
}

#[cfg(not(feature = "gpu"))]
fn wrap_shaders() {}

fn main() {
    wrap_shaders();

    // Generate C header for library clients
    cbindgen::Builder::new()
        .with_config(cbindgen::Config {
            cpp_compat: true,
            language: cbindgen::Language::C,
            include_guard: Some("TXKIT_H".to_owned()),
            includes: vec!["txkit_types.h".to_owned()],
            ..Default::default()
        })
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .rename_item("MethodBox", "Method")
        .rename_item("MappedImageDataReadBox", "MappedImageDataRead")
        .rename_item("MappedImageDataWriteBox", "MappedImageDataWrite")
        .generate()
        .expect("unable to generate C bindings")
        .write_to_file(
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("include")
                .join("txkit.h"),
        );
}
