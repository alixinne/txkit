#[cfg(all(feature = "gpu", feature = "wrap-shaders"))]
fn wrap_shaders() {
    use std::env;
    use tinygl_compiler::{WrappedItem, WrappedProgram};

    let debug_frag;
    let debug_prog;
    let whitenoise_frag;
    let whitenoise_prog;

    let mut compiler = tinygl_compiler::CompilerBuilder::default().build().unwrap();
    let mut wrapped: Vec<&dyn WrappedItem> = Vec::new();
    let mut set: Vec<&WrappedProgram> = Vec::new();

    let quad_vert = compiler.wrap_shader("shaders/quad.vert").unwrap();
    wrapped.push(&quad_vert);

    if cfg!(feature = "method-debug") {
        debug_frag = compiler.wrap_shader("shaders/debug.frag").unwrap();
        debug_prog = compiler
            .wrap_program(&[&quad_vert, &debug_frag], "debug")
            .unwrap();

        wrapped.push(&debug_frag);
        wrapped.push(&debug_prog);
        set.push(&debug_prog);
    }

    if cfg!(feature = "method-whitenoise") {
        whitenoise_frag = compiler.wrap_shader("shaders/whitenoise.frag").unwrap();
        whitenoise_prog = compiler
            .wrap_program(&[&quad_vert, &whitenoise_frag], "whitenoise")
            .unwrap();

        wrapped.push(&whitenoise_frag);
        wrapped.push(&whitenoise_prog);
        set.push(&whitenoise_prog);
    }

    let global_set = compiler.wrap_uniforms(&set[..], "global").unwrap();
    wrapped.push(&global_set);

    compiler
        .write_root_include(env::var("OUT_DIR").unwrap(), &wrapped[..])
        .unwrap();
}

#[cfg(not(all(feature = "gpu", feature = "wrap-shaders")))]
fn wrap_shaders() {}

fn main() {
    wrap_shaders();
}
