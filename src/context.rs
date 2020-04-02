#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
pub use cpu::*;

#[cfg(feature = "gpu")]
mod gpu;
#[cfg(feature = "gpu")]
pub use gpu::*;

#[cfg(not(feature = "cpu"))]
pub struct CpuContext;
#[cfg(not(feature = "gpu"))]
pub struct GpuContext;

/// txkit computing context
pub enum Context {
    Cpu(CpuContext),
    Gpu(GpuContext),
}

impl Context {
    #[cfg(feature = "cpu")]
    pub fn new_cpu() -> Result<Self, crate::method::Error> {
        CpuContext::new()
            .map_err(|e| crate::method::Error::ContextCreationFailed(e))
            .map(|s| Self::Cpu(s))
    }

    #[cfg(not(feature = "cpu"))]
    pub fn new_cpu() -> Result<Self, crate::method::Error> {
        Err(crate::method::Error::ContextNotSupported)
    }

    #[cfg(feature = "gpu")]
    pub fn new_gpu() -> Result<Self, crate::method::Error> {
        GpuContext::new()
            .map_err(|e| crate::method::Error::ContextCreationFailed(e))
            .map(|s| Self::Gpu(s))
    }

    #[cfg(not(feature = "gpu"))]
    pub fn new_gpu() -> Result<Self, crate::method::Error> {
        Err(crate::method::Error::ContextNotSupported)
    }

    pub fn cpu(&self) -> Option<&CpuContext> {
        match self {
            Self::Cpu(context) => Some(context),
            _ => None,
        }
    }

    pub fn gpu(&self) -> Option<&GpuContext> {
        match self {
            Self::Gpu(context) => Some(context),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! cpu_compute {
    ($cpu_context:ident, $tgt:ident, $idx:ident => $fn:expr) => {{
        use crate::image::IntoElementType;
        let mut data_mut = $tgt.data_mut()?;

        if let Some(data) = data_mut.as_u8_nd_array_mut() {
            $cpu_context.thread_pool.install(|| {
                par_azip!((index $idx, o in data) {
                    *o = $fn.into_u8();
                });
            });

            Ok(())
        } else if let Some(data) = data_mut.as_f32_nd_array_mut() {
            $cpu_context.thread_pool.install(|| {
                par_azip!((index $idx, o in data) {
                    *o = $fn.into_f32();
                });
            });

            Ok(())
        } else {
            Err(crate::method::Error::FormatNotSupported)
        }
    }}
}

#[no_mangle]
pub extern "C" fn txkit_context_new_cpu() -> *mut Context {
    crate::api::wrap_result(Context::new_cpu().map(Box::new).map(Box::into_raw))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn txkit_context_new_gpu() -> *mut Context {
    crate::api::wrap_result(Context::new_gpu().map(Box::new).map(Box::into_raw))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn txkit_context_destroy(ctx: *mut Context) {
    std::mem::drop(Box::from_raw(ctx))
}
