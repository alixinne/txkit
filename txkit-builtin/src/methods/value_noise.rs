use txkit_impl::{Method, ParamsFor};

#[derive(Clone, Copy, PartialEq, ParamsFor)]
#[repr(C)]
#[txkit(program = "ValueNoiseProgram")]
pub struct ValueNoiseParams {
    /// pseudo-random seed
    pub global_seed: u32,
    /// lattice scale (size in pixels)
    pub scale: f32,
}

impl Default for ValueNoiseParams {
    fn default() -> Self {
        Self {
            global_seed: 0,
            scale: 32.,
        }
    }
}

#[derive(Default, Method)]
#[txkit(
    gpu(
        name = "ValueNoiseGpu",
        program("shaders/quad.vert", "shaders/value_noise.frag"),
        method(run = "program", params = "ValueNoiseParams")
    ),
    method()
)]
pub struct ValueNoise {
    #[cfg(feature = "gpu")]
    gpu: Option<ValueNoiseGpu>,
}

impl ValueNoise {
    pub fn new() -> Self {
        Self::default()
    }
}
