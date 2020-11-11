macro_rules! compile_method {
    ($frag:ident, $prog:ident, $path:literal, $name:literal, $prefer_spirv:ident, $wrapped:ident, $set:ident, $compiler:ident, $reflector:ident, $quad_vert:ident) => {
        let sh = GlslObject::from_path($path, None)?.track_cargo();

        if $prefer_spirv {
            let sh = sh.compile(&mut $compiler)?.reflect_spirv(&$reflector)?;
            $frag = $compiler.wrap_shader(sh, true).unwrap();
        } else {
            let sh = sh
                .preprocess(&mut $compiler)?
                .compile(&mut $compiler)?
                .reflect_spirv(&$reflector)?;
            $frag = $compiler.wrap_shader(sh, false).unwrap();
        }

        $prog = $compiler
            .wrap_program(&[&$quad_vert, &$frag], $name)
            .unwrap();

        $wrapped.push(&$frag);
        $wrapped.push(&$prog);
        $set.push(&$prog);
    };
}

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
        compile_method!(
            debug_frag,
            debug_prog,
            "shaders/debug.frag",
            "debug",
            prefer_spirv,
            wrapped,
            set,
            compiler,
            reflector,
            quad_vert
        );
    }

    if cfg!(feature = "method-whitenoise") {
        compile_method!(
            whitenoise_frag,
            whitenoise_prog,
            "shaders/whitenoise.frag",
            "whitenoise",
            prefer_spirv,
            wrapped,
            set,
            compiler,
            reflector,
            quad_vert
        );
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
