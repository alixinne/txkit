use txkit_core::image::ImageDim;
use txkit_core::{context::Context, Error, Result};

#[derive(Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct WhiteNoiseParams {}

#[cfg(feature = "gpu")]
pub struct WhiteNoiseGpu {
    program: tinygl::wrappers::GlHandle<crate::shaders::WhiteNoiseProgram>,
}

#[cfg(feature = "gpu")]
impl WhiteNoiseGpu {
    pub fn new(ctx: &txkit_core::context::GpuContext) -> Result<Self> {
        let gl = ctx.gl().clone();

        Ok(Self {
            program: tinygl::wrappers::GlHandle::new(
                &gl,
                crate::shaders::WhiteNoiseProgram::build(&*gl)?,
            ),
        })
    }
}

#[cfg(feature = "gpu")]
impl txkit_core::method::TextureMethod for WhiteNoiseGpu {
    type Params = WhiteNoiseParams;
}

#[cfg(feature = "gpu")]
impl txkit_core::method::GpuMethod for WhiteNoiseGpu {
    fn compute_gpu(
        &mut self,
        ctx: &mut txkit_core::context::GpuContext,
        tgt: &mut txkit_core::image::gpu::GpuImageData,
        _params: &Self::Params,
    ) -> Result<()> {
        use tinygl::wrappers::ProgramCommonExt;
        use txkit_core::image::{ImageDataBase, ImageDimGpuExt};

        let dim = tgt.dim().into_cgmath();
        ctx.render_to_framebuffer(tgt, |gl, layer| {
            unsafe {
                self.program.use_program(gl);
            }

            // Common parameters
            self.program.set_i_resolution(gl, dim);
            self.program.set_i_layer(gl, layer);

            unsafe {
                gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
            }

            Ok(())
        })
    }
}

#[derive(Default)]
pub struct WhiteNoise {
    #[cfg(feature = "gpu")]
    gpu: Option<WhiteNoiseGpu>,
}

impl WhiteNoise {
    pub fn new() -> Self {
        Self::default()
    }

    fn compute_idx((k, j, i, l): (usize, usize, usize, usize), sz: ImageDim) -> f32 {
        let mut x = ((i + j * sz.width + k * sz.width * sz.height) * sz.channels + l) as u32;

        // Hash
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = (x >> 16) ^ x;

        // Convert to float
        f32::from_bits(0x7fu32 << 23 | x >> 9) - 1.0f32
    }
}

impl txkit_core::method::TextureMethod for WhiteNoise {
    type Params = WhiteNoiseParams;
}

#[cfg(feature = "cpu")]
impl txkit_core::method::CpuMethod for WhiteNoise {
    fn compute_cpu(
        &mut self,
        ctx: &mut txkit_core::context::CpuContext,
        tgt: &mut txkit_core::image::Image,
        _params: &Self::Params,
    ) -> Result<()> {
        use ::txkit_core::image::IntoElementType;
        use ndarray::par_azip;

        let dim = tgt.dim();
        let mut data_mut = tgt.data_mut()?;

        if let Some(data) = data_mut.as_u8_nd_array_mut() {
            ctx.thread_pool.install(|| {
                par_azip!((index idx, o in data) {
                    *o = Self::compute_idx(idx, dim).into_u8();
                });
            });

            Ok(())
        } else if let Some(data) = data_mut.as_f32_nd_array_mut() {
            ctx.thread_pool.install(|| {
                par_azip!((index idx, o in data) {
                    *o = Self::compute_idx(idx, dim).into_f32();
                });
            });

            Ok(())
        } else {
            Err(::txkit_core::Error::FormatNotSupported)
        }
    }
}

impl txkit_core::method::Method for WhiteNoise {
    fn compute(
        &mut self,
        ctx: &mut txkit_core::context::Context,
        tgt: &mut txkit_core::image::Image,
        params: Option<&dyn std::any::Any>,
    ) -> Result<()> {
        let mut default_params = None;
        let params = txkit_core::method::downcast_params(params, &mut default_params)?;

        match ctx {
            #[cfg(feature = "gpu")]
            Context::Gpu(gpu_context) => tgt
                .as_gpu_image_mut()
                .ok_or_else(|| Error::FormatNotSupported)
                .and_then(|tgt| {
                    use txkit_core::method::GpuMethod;

                    // Initialize GPU if needed
                    if let None = self.gpu {
                        self.gpu = Some(WhiteNoiseGpu::new(gpu_context)?);
                    }

                    // Compute result using initialized GPU resources
                    let gpu = self.gpu.as_mut().unwrap();
                    gpu.compute_gpu(gpu_context, tgt, params)
                }),
            #[cfg(not(feature = "gpu"))]
            Context::Gpu(_) => Err(Error::ContextNotSupported),
            #[cfg(feature = "cpu")]
            Context::Cpu(cpu_context) => {
                use txkit_core::method::CpuMethod;

                self.compute_cpu(cpu_context, tgt, params)
            }
            #[cfg(not(feature = "cpu"))]
            Context::Cpu(_) => Err(Error::ContextNotSupported),
        }
    }
}
