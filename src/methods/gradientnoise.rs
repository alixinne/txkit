decl_method! {
    name Gradientnoise;
    gpu {
        name GradientnoiseGpu;
        program crate::shaders::GradientnoiseProgram;
        prepare (_gl, _program, _params) => {
            Ok(())
        };
    }
    params {
        GradientnoiseParams {
        }
    }
}
