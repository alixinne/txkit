#[cfg(all(feature = "gpu", feature = "wrap-shaders"))]
fn wrap_shaders() -> anyhow::Result<()> {
    use std::env;
    use std::path::PathBuf;
    use tinygl_compiler::{
        codegen::WrappedItem, model::GlslObject, reflect, Compiler, WrappedProgram,
    };

    let debug_frag;
    let debug_prog;
    let whitenoise_frag;
    let whitenoise_prog;

    let mut compiler = Compiler::new(false, None).unwrap().with_shaderc();
    let reflector = reflect::SpirVBackend::new();

    let mut wrapped: Vec<&dyn WrappedItem> = Vec::new();
    let mut set: Vec<&WrappedProgram> = Vec::new();

    let quad_vert = GlslObject::from_path("shaders/quad.vert", None)?
        .track_cargo()
        .compile(&mut compiler)?
        .reflect_spirv(&reflector)?;

    let prefer_spirv = cfg!(feature = "tinygl/opengl46");
    let quad_vert = compiler.wrap_shader(quad_vert, prefer_spirv)?;

    wrapped.push(&quad_vert);

    if cfg!(feature = "method-debug") {
        let sh = GlslObject::from_path("shaders/debug.frag", None)?
            .track_cargo()
            .compile(&mut compiler)?
            .reflect_spirv(&reflector)?;
        debug_frag = compiler.wrap_shader(sh, prefer_spirv).unwrap();
        debug_prog = compiler
            .wrap_program(&[&quad_vert, &debug_frag], "debug")
            .unwrap();

        wrapped.push(&debug_frag);
        wrapped.push(&debug_prog);
        set.push(&debug_prog);
    }

    if cfg!(feature = "method-whitenoise") {
        let sh = GlslObject::from_path("shaders/whitenoise.frag", None)?
            .track_cargo()
            .compile(&mut compiler)?
            .reflect_spirv(&reflector)?;
        whitenoise_frag = compiler.wrap_shader(sh, prefer_spirv).unwrap();
        whitenoise_prog = compiler
            .wrap_program(&[&quad_vert, &whitenoise_frag], "whitenoise")
            .unwrap();

        wrapped.push(&whitenoise_frag);
        wrapped.push(&whitenoise_prog);
        set.push(&whitenoise_prog);
    }

    let global_set = compiler.wrap_uniforms(&set[..], "global").unwrap();
    wrapped.push(&global_set);

    tinygl_compiler::codegen::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("shaders.rs"),
        &wrapped,
    )?;

    Ok(())
}

#[cfg(not(all(feature = "gpu", feature = "wrap-shaders")))]
fn wrap_shaders() -> Result<(), anyhow::Error> {
    Ok(())
}

fn main() {
    wrap_shaders().expect("failed to wrap shaders");
}
