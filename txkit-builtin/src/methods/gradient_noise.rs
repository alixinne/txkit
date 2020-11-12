decl_method! {
    name GradientNoise;
    gpu {
        name GradientNoiseGpu;
        program crate::shaders::GradientNoiseProgram;
        prepare (_gl, _program, _params) => {
            Ok(())
        };
    }
    params {
        GradientNoiseParams {
        }
    }
}
