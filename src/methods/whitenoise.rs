use std::rc::Rc;

use ndarray::par_azip;

use tinygl::prelude::*;
use tinygl::wrappers::GlHandle;

use crate::context::Context;
use crate::image::Image;
use crate::method::*;

#[derive(Default)]
pub struct Whitenoise {
    gpu: Option<WhitenoiseGpu>,
}

struct WhitenoiseGpu {
    program: GlHandle<crate::shaders::WhitenoiseProgram>,
}

impl WhitenoiseGpu {
    fn new(gl: &Rc<tinygl::Context>) -> Result<Self, String> {
        Ok(Self {
            program: GlHandle::new(gl, crate::shaders::WhitenoiseProgram::build(gl)?),
        })
    }
}

impl Whitenoise {
    pub fn new() -> Self {
        Self::default()
    }

    fn hash_idx(
        (j, i, k, l): (usize, usize, usize, usize),
        sz: (usize, usize, usize, usize),
    ) -> f32 {
        let mut x = ((i + j * sz.0 + k * sz.0 * sz.1) * sz.3 + l) as u32;

        // Hash
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = (x >> 16) ^ x;

        // Convert to float
        f32::from_bits(0x7fu32 << 23 | x >> 9) - 1.0f32
    }
}

impl Method for Whitenoise {
    fn compute(&mut self, ctx: &mut Context, tgt: &mut Image) -> Result<(), Error> {
        match ctx {
            Context::Gpu(gpu_context) => {
                // Initialize GPU program
                if self.gpu.is_none() {
                    self.gpu = Some(
                        WhitenoiseGpu::new(&gpu_context.gl)
                            .map_err(|e| crate::method::Error::MethodInitializationFailed(e))?,
                    );
                }

                let dim = cgmath::vec3(tgt.width() as u32, tgt.height() as u32, tgt.depth() as u32);
                let gpu = self.gpu.as_ref().unwrap();

                gpu_context.render_to_framebuffer(tgt, |gl| {
                    gpu.program.use_program(gl);
                    gpu.program.set_i_resolution(gl, dim);

                    unsafe {
                        gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
                    }

                    Ok(())
                })
            }
            Context::Cpu(_cpu_context) => match tgt {
                Image::UInt8(ref mut data) => {
                    let sz = data.dim();

                    par_azip!((index idx, o in data) {
                        *o = (Self::hash_idx(idx, sz) * 255.0f32) as u8;
                    });

                    Ok(())
                }
                Image::Float32(ref mut data) => {
                    let sz = data.dim();

                    par_azip!((index idx, o in data) {
                        *o = Self::hash_idx(idx, sz);
                    });

                    Ok(())
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitenoise_cpu() {
        let mut ctx = Context::new_cpu().unwrap();
        let mut img = Image::new_u8(16, 16, 4);

        assert_eq!(Ok(()), Whitenoise::new().compute(&mut ctx, &mut img));
    }
}
