use txkit_core::{context::Context, Error, Result};

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DebugParams {
    pub alpha_value: f32,
}

impl Default for DebugParams {
    fn default() -> Self {
        Self { alpha_value: 1.0 }
    }
}

#[cfg(feature = "gpu")]
pub struct DebugGpu {
    program: tinygl::wrappers::GlHandle<crate::shaders::DebugProgram>,
}

#[cfg(feature = "gpu")]
impl DebugGpu {
    pub fn new(ctx: &txkit_core::context::GpuContext) -> Result<Self> {
        let gl = ctx.gl().clone();

        Ok(Self {
            program: tinygl::wrappers::GlHandle::new(
                &gl,
                crate::shaders::DebugProgram::build(&*gl)?,
            ),
        })
    }
}

#[cfg(feature = "gpu")]
impl txkit_core::method::TextureMethod for DebugGpu {
    type Params = DebugParams;
}

#[cfg(feature = "gpu")]
impl txkit_core::method::GpuMethod for DebugGpu {
    fn compute_gpu(
        &mut self,
        ctx: &mut txkit_core::context::GpuContext,
        tgt: &mut txkit_core::image::gpu::GpuImageData,
        params: &Self::Params,
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

            // Method parameters
            self.program.set_alpha_value(gl, params.alpha_value);

            unsafe {
                gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
            }

            Ok(())
        })
    }
}

#[derive(Default)]
pub struct Debug {
    #[cfg(feature = "gpu")]
    gpu: Option<DebugGpu>,
}

impl Debug {
    pub fn new() -> Self {
        Self::default()
    }

    fn compute_idx((k, j, i, l): (usize, usize, usize, usize), params: &DebugParams) -> f32 {
        match l {
            0 => i as f32,
            1 => j as f32,
            2 => k as f32,
            3 => params.alpha_value,
            _ => unreachable!(),
        }
    }
}

impl txkit_core::method::TextureMethod for Debug {
    type Params = DebugParams;
}

#[cfg(feature = "cpu")]
impl txkit_core::method::CpuMethod for Debug {
    fn compute_cpu(
        &mut self,
        ctx: &mut txkit_core::context::CpuContext,
        tgt: &mut txkit_core::image::Image,
        params: &Self::Params,
    ) -> Result<()> {
        use ::txkit_core::image::IntoElementType;
        use ndarray::par_azip;

        let mut data_mut = tgt.data_mut()?;

        if let Some(data) = data_mut.as_u8_nd_array_mut() {
            ctx.thread_pool.install(|| {
                par_azip!((index idx, o in data) {
                    *o = Self::compute_idx(idx, params).into_u8();
                });
            });

            Ok(())
        } else if let Some(data) = data_mut.as_f32_nd_array_mut() {
            ctx.thread_pool.install(|| {
                par_azip!((index idx, o in data) {
                    *o = Self::compute_idx(idx, params).into_f32();
                });
            });

            Ok(())
        } else {
            Err(::txkit_core::Error::FormatNotSupported)
        }
    }
}

impl txkit_core::method::Method for Debug {
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
                        self.gpu = Some(DebugGpu::new(gpu_context)?);
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
