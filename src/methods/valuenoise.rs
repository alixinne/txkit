decl_method! {
    name Valuenoise;
    gpu {
        name ValuenoiseGpu;
        program crate::shaders::ValuenoiseProgram;
        prepare (_gl, _program, _params) => {
            Ok(())
        };
    }
    params {
        ValuenoiseParams {
        }
    }
}
