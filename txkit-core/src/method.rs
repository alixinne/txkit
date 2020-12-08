use std::any::Any;

use crate::context::Context;
use crate::error::*;
use crate::image::Image;

mod registry;
pub use registry::*;

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
pub use self::cpu::*;

#[cfg(feature = "gpu-core")]
mod gpu;
#[cfg(feature = "gpu-core")]
pub use self::gpu::*;

/// Try to downcast a generic params struct into the target params type
pub fn downcast_params<'u, U: Default + 'static>(
    params: Option<&'u dyn std::any::Any>,
    default_params: &'u mut Option<U>,
) -> Result<&'u U> {
    Ok(match params {
        Some(params) => {
            if let Some(p) = params.downcast_ref() {
                p
            } else if let Some(buf) = params.downcast_ref::<&[u8]>() {
                if buf.len() != std::mem::size_of::<U>() {
                    return Err(Error::InvalidParameters);
                }

                unsafe { &*(buf.as_ptr() as *const U) }
            } else {
                return Err(Error::InvalidParameters);
            }
        }
        None => {
            *default_params = Some(U::default());
            default_params.as_ref().unwrap()
        }
    })
}

/// Generic interface to a procedural texturing method
pub trait Method {
    fn compute(
        &mut self,
        ctx: &mut Context,
        tgt: &mut Image,
        params: Option<&dyn Any>,
    ) -> Result<()>;
}
