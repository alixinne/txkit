#[macro_export]
macro_rules! decl_method {
    {
        name $name:ident;
        gpu {
            name $gpu_name:ident;
            program $gpu_program:ty;
        }
        cpu (($k:ident, $j:ident, $i:ident, $l:ident), $dim:ident) -> $t:ty => $cpu:expr
    } => {
        use crate::context::Context;
        use crate::image::prelude::*;
        use crate::method::*;

        #[derive(Default)]
        pub struct $name {
            #[cfg(feature = "gpu")]
            gpu: Option<$gpu_name>,
        }

        #[cfg(feature = "gpu")]
        struct $gpu_name {
            program: tinygl::wrappers::GlHandle<$gpu_program>,
        }

        #[cfg(feature = "gpu")]
        impl $gpu_name {
            fn new(gl: &std::rc::Rc<tinygl::Context>) -> Result<Self, String> {
                Ok(Self {
                    program: tinygl::wrappers::GlHandle::new(gl, <$gpu_program>::build(gl)?)
                })
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            #[cfg(feature = "cpu")]
            fn compute_idx(($k, $j, $i, $l): (usize, usize, usize, usize), $dim: crate::image::ImageDim) -> $t {
                $cpu
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
                    self.gpu = Some($gpu_name::new(&gpu_context.gl)
                        .map_err(|e| crate::method::Error::MethodInitializationFailed(e))?,
                    )
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
                _gpu_context: &mut crate::context::GpuContext,
                _tgt: &mut Image,
            ) -> Result<(), Error> {
                Err(crate::method::Error::ContextNotSupported)
            }
        }

        #[allow(unused_variables)]
        impl Method for $name {
            fn compute(&mut self, ctx: &mut Context, tgt: &mut Image) -> Result<(), Error> {
                match ctx {
                    Context::Gpu(gpu_context) => self.compute_gpu(gpu_context, tgt),
                    Context::Cpu(cpu_context) => {
                        let sz = tgt.dim();
                        cpu_compute!(cpu_context, tgt, idx => Self::compute_idx(idx, sz))
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

                assert_eq!(Ok(()), $name::new().compute(&mut ctx, &mut img));
            }
        }
    }
}
