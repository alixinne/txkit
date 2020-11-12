decl_method! {
    name Debug;
    gpu {
        name DebugGpu;
        program crate::shaders::DebugProgram;
        prepare (gl, program, params) => {
            program.set_alpha_value(gl, params.alpha_value);

            Ok(())
        };
    }
    params {
        DebugParams {
            alpha_value: f32 = 1.0,
        }
    }
    cpu ((k, j, i, l), _dim, params) -> f32 => match l {
        0 => i as f32,
        1 => j as f32,
        2 => k as f32,
        3 => params.alpha_value,
        _ => unreachable!(),
    }
}
