#[macro_export]
macro_rules! decl_method {
    {
        name $name:ident;
        gpu {
            name $gpu_name:ident;
            program $gpu_program:ty;
            prepare ($gl:ident, $prepare_program:ident, $prepare_params:ident) => $prepare_code:block;
        }
        params {
            $params_name:ident {
                $($param_name:ident: $param_type:ty = $default_value:expr,)*
            }
        }
        cpu (($k:ident, $j:ident, $i:ident, $l:ident), $dim:ident, $params:ident) -> $t:ty => $cpu:expr
    } => {
        use crate::context::Context;
        use crate::image::prelude::*;
        use crate::method::*;

        #[derive(Default)]
        pub struct $name {
            #[cfg(feature = "gpu-core")]
            gpu: Option<$gpu_name>,
        }

        #[derive(Clone, PartialEq)]
        #[repr(C)]
        pub struct $params_name {
            $($param_name: $param_type),*
        }

        impl Default for $params_name {
            fn default() -> Self {
                Self {
                    $($param_name: $default_value),*
                }
            }
        }

        #[cfg(feature = "gpu-core")]
        struct $gpu_name {
            program: tinygl::wrappers::GlHandle<$gpu_program>,
        }

        #[cfg(feature = "gpu-core")]
        impl $gpu_name {
            fn new(gl: &std::rc::Rc<tinygl::Context>) -> crate::Result<Self> {
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
            fn compute_idx(($k, $j, $i, $l): (usize, usize, usize, usize), $dim: crate::image::ImageDim, $params: &$params_name) -> $t {
                $cpu
            }

            #[cfg(feature = "gpu-core")]
            fn prepare_gpu($gl: &tinygl::Context, $prepare_program: &$gpu_program, $prepare_params: &$params_name) -> crate::Result<()> {
                $prepare_code
            }

            #[cfg(feature = "gpu-core")]
            fn compute_gpu(
                &mut self,
                gpu_context: &mut crate::context::GpuContext,
                tgt: &mut Image,
                params: &$params_name,
            ) -> crate::Result<()> {
                use tinygl::prelude::*;

                // Initialize GPU program
                if self.gpu.is_none() {
                    self.gpu = Some($gpu_name::new(&gpu_context.gl)?);
                }

                let dim = tgt.dim().into_cgmath();
                let gpu = self.gpu.as_ref().unwrap();

                tgt.as_gpu_image_mut()
                    .ok_or(crate::Error::FormatNotSupported)
                    .and_then(|tgt| {
                        gpu_context.render_to_framebuffer(tgt, |gl, layer| {
                            unsafe { gpu.program.use_program(gl); }
                            gpu.program.set_i_resolution(gl, dim);
                            gpu.program.set_i_layer(gl, layer);

                            Self::prepare_gpu(&*gl, &gpu.program, params)?;

                            unsafe {
                                gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
                            }

                            Ok(())
                        })
                    })
            }

            #[cfg(not(feature = "gpu-core"))]
            fn compute_gpu(
                &mut self,
                _gpu_context: &mut crate::context::GpuContext,
                _tgt: &mut Image,
                _params: &$params_name,
            ) -> crate::Result<()> {
                Err(crate::Error::ContextNotSupported)
            }
        }

        #[allow(unused_variables)]
        impl Method for $name {
            fn compute(&mut self, ctx: &mut Context, tgt: &mut Image, params: Option<&dyn std::any::Any>) -> crate::Result<()> {
                let default_params;
                let params: &$params_name = match params {
                    Some(params) => {
                        if let Some(p) = params.downcast_ref() {
                            p
                        } else if let Some(buf) = params.downcast_ref::<&[u8]>() {
                            if buf.len() != std::mem::size_of::<$params_name>() {
                                return Err(crate::Error::InvalidParameters);
                            }

                            unsafe {
                                &*(buf.as_ptr() as *const $params_name)
                            }
                        } else {
                            return Err(crate::Error::InvalidParameters);
                        }
                    }
                    None => {
                        default_params = $params_name::default();
                        &default_params
                    }
                };

                match ctx {
                    Context::Gpu(gpu_context) => self.compute_gpu(gpu_context, tgt, &params),
                    Context::Cpu(cpu_context) => {
                        let sz = tgt.dim();
                        cpu_compute!(cpu_context, tgt, idx => Self::compute_idx(idx, sz, &params))
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

                assert!($name::new().compute(&mut ctx, &mut img, None).is_ok());
            }
        }
    }
}
