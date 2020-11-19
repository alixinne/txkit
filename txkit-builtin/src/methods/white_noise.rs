use txkit_core::image::ImageDim;
use txkit_impl::{Method, ParamsFor};

#[derive(Default, Clone, Copy, PartialEq, ParamsFor)]
#[repr(C)]
#[txkit(program = "WhiteNoiseProgram")]
pub struct WhiteNoiseParams {}

#[derive(Default, Method)]
#[txkit(
    gpu(
        name = "WhiteNoiseGpu",
        program("shaders/quad.vert", "shaders/white_noise.frag"),
        method(run = "program", params = "WhiteNoiseParams")
    ),
    cpu(method(iter = "Self::compute_idx", params = "WhiteNoiseParams")),
    method()
)]
pub struct WhiteNoise {
    #[cfg(feature = "gpu")]
    gpu: Option<WhiteNoiseGpu>,
}

impl WhiteNoise {
    pub fn new() -> Self {
        Self::default()
    }

    fn compute_idx(
        (k, j, i, l): (usize, usize, usize, usize),
        sz: ImageDim,
        _params: &WhiteNoiseParams,
    ) -> f32 {
        let mut x = ((i + j * sz.width + k * sz.width * sz.height) * sz.channels + l) as u32;

        // Hash
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = (x >> 16) ^ x;

        // Convert to float
        f32::from_bits(0x7fu32 << 23 | x >> 9) - 1.0f32
    }
}
