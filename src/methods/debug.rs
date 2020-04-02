decl_method! {
    name Debug;
    gpu {
        name DebugGpu;
        program crate::shaders::DebugProgram;
    }
    cpu ((_k, j, i, l), _dim) -> f32 => match l {
        0 => i as f32,
        1 => j as f32,
        2 => 0.0,
        3 => 1.0,
        _ => unreachable!(),
    }
}
