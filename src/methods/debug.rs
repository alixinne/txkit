use std::rc::Rc;

use ndarray::par_azip;

use tinygl::prelude::*;
use tinygl::wrappers::GlHandle;

use crate::context::Context;
use crate::image::gpu::ImageDimGpuExt;
use crate::image::Image;
use crate::method::*;

#[derive(Default)]
pub struct Debug {
    gpu: Option<DebugGpu>,
}

struct DebugGpu {
    program: GlHandle<crate::shaders::DebugProgram>,
}

impl DebugGpu {
    fn new(gl: &Rc<tinygl::Context>) -> Result<Self, String> {
        Ok(Self {
            program: GlHandle::new(gl, crate::shaders::DebugProgram::build(gl)?),
        })
    }
}

impl Debug {
    pub fn new() -> Self {
        Self::default()
    }

    fn debug_idx((j, i, _k, l): (usize, usize, usize, usize)) -> f32 {
        match l {
            0 => i as f32,
            1 => j as f32,
            2 => 0.0,
            3 => 1.0,
            _ => unreachable!(),
        }
    }
}

impl Method for Debug {
    fn compute(&mut self, ctx: &mut Context, tgt: &mut Image) -> Result<(), Error> {
        match ctx {
            Context::Gpu(gpu_context) => {
                // Initialize GPU program
                if self.gpu.is_none() {
                    self.gpu = Some(
                        DebugGpu::new(&gpu_context.gl)
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
            Context::Cpu(cpu_context) => {
                let mut data_mut = tgt.data_mut()?;

                if let Some(data) = data_mut.as_u8_nd_array_mut() {
                    cpu_context.thread_pool.install(|| {
                        par_azip!((index idx, o in data) {
                            *o = (Self::debug_idx(idx) * 255.0f32) as u8;
                        });
                    });

                    Ok(())
                } else if let Some(data) = data_mut.as_f32_nd_array_mut() {
                    cpu_context.thread_pool.install(|| {
                        par_azip!((index idx, o in data) {
                            *o = Self::debug_idx(idx);
                        });
                    });

                    Ok(())
                } else {
                    Err(crate::method::Error::FormatNotSupported)
                }
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

        assert_eq!(Ok(()), Debug::new().compute(&mut ctx, &mut img));
    }
}
