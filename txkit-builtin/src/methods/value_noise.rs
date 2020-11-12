decl_method! {
    name ValueNoise;
    gpu {
        name ValueNoiseGpu;
        program crate::shaders::ValueNoiseProgram;
        prepare (_gl, _program, _params) => {
            Ok(())
        };
    }
    params {
        ValueNoiseParams {
        }
    }
}
