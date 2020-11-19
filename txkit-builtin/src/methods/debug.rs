use txkit_core::image::ImageDim;
use txkit_impl::{Method, ParamsFor};

#[derive(Clone, Copy, PartialEq, ParamsFor)]
#[repr(C)]
#[txkit(program = "DebugProgram")]
pub struct DebugParams {
    pub alpha_value: f32,
}

impl Default for DebugParams {
    fn default() -> Self {
        Self { alpha_value: 1.0 }
    }
}

#[derive(Default, Method)]
#[txkit(
    gpu(
        name = "DebugGpu",
        program("shaders/quad.vert", "shaders/debug.frag"),
        method(run = "program", params = "DebugParams")
    ),
    cpu(method(iter = "Self::compute_idx", params = "DebugParams")),
    method()
)]
pub struct Debug {
    #[cfg(feature = "gpu")]
    gpu: Option<DebugGpu>,
}

impl Debug {
    pub fn new() -> Self {
        Self::default()
    }

    fn compute_idx(
        (k, j, i, l): (usize, usize, usize, usize),
        _dim: ImageDim,
        params: &DebugParams,
    ) -> f32 {
        match l {
            0 => i as f32,
            1 => j as f32,
            2 => k as f32,
            3 => params.alpha_value,
            _ => unreachable!(),
        }
    }
}
