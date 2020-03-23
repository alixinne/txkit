use std::env;
use std::path::PathBuf;

#[cfg(feature = "gpu")]
fn wrap_shaders() {
    let mut compiler = tinygl_compiler::CompilerBuilder::default().build().unwrap();

    compiler.wrap_shader("shaders/debug.frag").unwrap();
    compiler.wrap_shader("shaders/quad.vert").unwrap();
    compiler.wrap_shader("shaders/whitenoise.frag").unwrap();

    compiler
        .wrap_program(&["shaders/quad.vert", "shaders/debug.frag"], "debug")
        .unwrap();

    compiler
        .wrap_program(
            &["shaders/quad.vert", "shaders/whitenoise.frag"],
            "whitenoise",
        )
        .unwrap();

    compiler
        .wrap_uniforms(&["debug", "whitenoise"], "global")
        .unwrap();

    compiler.write_root_include().unwrap();
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
            ..Default::default()
        })
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .generate()
        .expect("unable to generate C bindings")
        .write_to_file(
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("include")
                .join("txkit.h"),
        );
}
