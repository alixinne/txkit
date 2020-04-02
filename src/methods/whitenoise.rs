use crate::context::Context;
use crate::image::prelude::*;
use crate::method::*;

#[derive(Default)]
pub struct Whitenoise {
    #[cfg(feature = "gpu")]
    gpu: Option<WhitenoiseGpu>,
}

#[cfg(feature = "gpu")]
struct WhitenoiseGpu {
    program: tinygl::wrappers::GlHandle<crate::shaders::WhitenoiseProgram>,
}

#[cfg(feature = "gpu")]
impl WhitenoiseGpu {
    fn new(gl: &std::rc::Rc<tinygl::Context>) -> Result<Self, String> {
        Ok(Self {
            program: tinygl::wrappers::GlHandle::new(
                gl,
                crate::shaders::WhitenoiseProgram::build(gl)?,
            ),
        })
    }
}

impl Whitenoise {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "cpu")]
    fn hash_idx((k, j, i, l): (usize, usize, usize, usize), sz: crate::image::ImageDim) -> f32 {
        let mut x = ((i + j * sz.width + k * sz.width * sz.height) * sz.channels + l) as u32;

        // Hash
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = (x >> 16) ^ x;

        // Convert to float
        f32::from_bits(0x7fu32 << 23 | x >> 9) - 1.0f32
    }

    #[cfg(feature = "gpu")]
    fn compute_gpu(
        &mut self,
        gpu_context: &mut crate::context::GpuContext,
        tgt: &mut Image,
    ) -> Result<(), Error> {
        use tinygl::prelude::*;

        // Initialize GPU program
        if self.gpu.is_none() {
            self.gpu = Some(
                WhitenoiseGpu::new(&gpu_context.gl)
                    .map_err(|e| crate::method::Error::MethodInitializationFailed(e))?,
            );
        }

        let dim = tgt.dim().into_cgmath();
        let gpu = self.gpu.as_ref().unwrap();

        tgt.as_gpu_image_mut()
            .ok_or(crate::method::Error::FormatNotSupported)
            .and_then(|tgt| {
                gpu_context.render_to_framebuffer(tgt, |gl| {
                    gpu.program.use_program(gl);
                    gpu.program.set_i_resolution(gl, dim);

                    unsafe {
                        gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
                    }

                    Ok(())
                })
            })
    }

    #[cfg(not(feature = "gpu"))]
    fn compute_gpu(
        &mut self,
        _ctx: &mut crate::context::GpuContext,
        _tgt: &mut Image,
    ) -> Result<(), Error> {
        Err(crate::method::Error::ContextNotSupported)
    }
}

impl Method for Whitenoise {
    fn compute(&mut self, ctx: &mut Context, tgt: &mut Image) -> Result<(), Error> {
        match ctx {
            Context::Gpu(gpu_context) => self.compute_gpu(gpu_context, tgt),
            Context::Cpu(cpu_context) => {
                let sz = tgt.dim();
                cpu_compute!(cpu_context, tgt, idx => Self::hash_idx(idx, sz))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::{ImageDataType, ImageDim};

    #[test]
    fn debug_cpu() {
        let mut ctx = Context::new_cpu().unwrap();
        let mut img = Image::new_cpu(ImageDim::new(16, 16, 4), ImageDataType::UInt8);

        assert_eq!(Ok(()), Whitenoise::new().compute(&mut ctx, &mut img));
    }
}
