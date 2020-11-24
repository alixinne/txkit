use txkit_impl::{Method, ParamsFor};

#[derive(Default, Clone, Copy, PartialEq, ParamsFor)]
#[repr(C)]
#[txkit(program = "GradientNoiseProgram")]
pub struct GradientNoiseParams {
    /// pseudo-random seed
    pub global_seed: u32,
}

#[derive(Default, Method)]
#[txkit(
    gpu(
        name = "GradientNoiseGpu",
        program("shaders/quad.vert", "shaders/gradient_noise.frag"),
        method(run = "program", params = "GradientNoiseParams")
    ),
    method()
)]
pub struct GradientNoise {
    #[cfg(feature = "gpu")]
    gpu: Option<GradientNoiseGpu>,
}

impl GradientNoise {
    pub fn new() -> Self {
        Self::default()
    }
}
