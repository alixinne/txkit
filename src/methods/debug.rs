decl_method! {
    name Debug;
    gpu {
        name DebugGpu;
        program crate::shaders::DebugProgram;
    }
    cpu ((k, j, i, l), _dim) -> f32 => match l {
        0 => i as f32,
        1 => j as f32,
        2 => k as f32,
        3 => 1.0,
        _ => unreachable!(),
    }
}
