use txkit_impl::{Method, ParamsFor};

#[derive(Clone, Copy, PartialEq, ParamsFor)]
#[repr(C)]
#[txkit(program = "GradientNoiseProgram")]
pub struct GradientNoiseParams {
    /// pseudo-random seed
    pub global_seed: u32,
    /// lattice scale (size in pixels)
    pub scale: f32,
    /// stats mode (0: normal, 1: process, 2: lookat)
    pub stats_mode: i32,
    /// look-at parameter (if stats_mode == lookat) in [0, 1]^2
    pub stats_look_at: cgmath::Vector2<f32>,
}

impl Default for GradientNoiseParams {
    fn default() -> Self {
        Self {
            global_seed: 0,
            scale: 32.,
            stats_mode: 0,
            stats_look_at: cgmath::vec2(0., 0.),
        }
    }
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
