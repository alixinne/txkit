use txkit_core::io::ImageIo;
use txkit_impl::{Method, ParamsFor};

/// Phasor: complex sum divided by the kernel count in R and G
pub const PHASOR_PROFILE_COMPLEX: i32 = 0;
/// Phasor: real part of the complex sum divided by the kernel count in R
pub const PHASOR_PROFILE_REAL: i32 = 1;
/// Phasor: imaginary part of the complex sum divided by the kernel count in R
pub const PHASOR_PROFILE_IMAG: i32 = 2;
/// Phasor: phasor sine wave
pub const PHASOR_PROFILE_SIN: i32 = 3;
/// Phasor: phasor sawtooth wave
pub const PHASOR_PROFILE_SAW: i32 = 4;
/// Phasor: show impulse locations
pub const PHASOR_PROFILE_IMPULSES: i32 = 5;

/// Phasor: no kernel weights
pub const PHASOR_WEIGHTS_NONE: i32 = 0;
/// Phasor: bernoulli kernel weights
pub const PHASOR_WEIGHTS_BERNOULLI: i32 = 1;
/// Phasor: uniform kernel weights
pub const PHASOR_WEIGHTS_UNIFORM: i32 = 2;

/// Phasor: constant number of impulses per cell
pub const PHASOR_POINTS_STRAT_POSSION: i32 = 0;
/// Phasor: Poisson number of impulses per cell
pub const PHASOR_POINTS_POISSON: i32 = 1;
/// Phasor: rectangular jittered grid
pub const PHASOR_POINTS_RECT_JITTERED: i32 = 2;
/// Phasor: hexagonal jittered grid
pub const PHASOR_POINTS_HEX_JITTERED: i32 = 3;

#[derive(Clone, PartialEq, ParamsFor)]
#[repr(C)]
#[txkit(program = "PhasorNoiseProgram")]
pub struct PhasorNoiseParams {
    /// pseudo-random seed
    pub global_seed: u32,
    /// lattice scale (size in pixels)
    pub scale: f32,
    /// stats mode (0: normal, 1: process, 2: lookat)
    pub stats_mode: i32,
    /// look-at parameter (if stats_mode == lookat) in [0, 1]^2
    pub stats_look_at: cgmath::Vector2<f32>,

    /// cell lookahead: number of cells to check for contributions
    pub noise_lookahead: i32,
    /// kernel count per cell
    pub kernel_count: i32,
    /// noise profile function
    pub noise_profile: i32,
    /// noise weights
    pub noise_weights: i32,
    /// point distribution
    pub noise_point_distribution: i32,

    /// noise frequency (in oscillations / noise cell)
    pub noise_frequency: f32,
    /// noise angle (in radians)
    pub noise_angle: f32,

    /// jittering amount, 0 = no random, 1 = full subcell random
    pub jitter_amount: f32,
    /// max jittering subcells, 0 = no limit
    pub jitter_max: i32,

    /// texture inputs
    #[texture_io(frequency_orientation_field)]
    pub io: Box<ImageIo>,
}

impl Default for PhasorNoiseParams {
    fn default() -> Self {
        Self {
            global_seed: 0,
            scale: 32.,
            stats_mode: 0,
            stats_look_at: cgmath::vec2(0., 0.),
            noise_lookahead: 1,
            kernel_count: 8,
            noise_profile: PHASOR_PROFILE_SIN,
            noise_weights: PHASOR_WEIGHTS_NONE,
            noise_point_distribution: PHASOR_POINTS_STRAT_POSSION,
            noise_frequency: 4.,
            noise_angle: 0.,
            jitter_amount: 1.,
            jitter_max: 0,
            io: Box::new(Default::default()),
        }
    }
}

#[derive(Default, Method)]
#[txkit(
    gpu(
        name = "PhasorNoiseGpu",
        program("shaders/quad.vert", "shaders/phasor_noise.frag"),
        method(run = "program", params = "PhasorNoiseParams")
    ),
    method()
)]
pub struct PhasorNoise {
    #[cfg(feature = "gpu")]
    gpu: Option<PhasorNoiseGpu>,
}

impl PhasorNoise {
    pub fn new() -> Self {
        Self::default()
    }
}
