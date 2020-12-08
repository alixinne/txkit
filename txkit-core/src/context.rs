use crate::Result;

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
pub use cpu::*;

#[cfg(feature = "gpu-core")]
mod gpu;
#[cfg(feature = "gpu-core")]
pub use gpu::*;

#[cfg(not(feature = "cpu"))]
pub struct CpuContext;
#[cfg(not(feature = "gpu-core"))]
pub struct GpuContext;

/// txkit computing context
pub enum Context {
    Cpu(CpuContext),
    Gpu(GpuContext),
}

impl Context {
    #[cfg(feature = "cpu")]
    pub fn new_cpu() -> Result<Self> {
        CpuContext::new().map(|s| Self::Cpu(s))
    }

    #[cfg(not(feature = "cpu"))]
    pub fn new_cpu() -> Result<Self> {
        Err(crate::Error::ContextNotSupported)
    }

    #[cfg(feature = "gpu-core")]
    pub fn new_gpu() -> Result<Self> {
        GpuContext::new().map(|s| Self::Gpu(s))
    }

    #[cfg(not(feature = "gpu-core"))]
    pub fn new_gpu() -> Result<Self> {
        Err(crate::Error::ContextNotSupported)
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

    pub fn cpu_mut(&mut self) -> Option<&mut CpuContext> {
        match self {
            Self::Cpu(context) => Some(context),
            _ => None,
        }
    }

    pub fn gpu_mut(&mut self) -> Option<&mut GpuContext> {
        match self {
            Self::Gpu(context) => Some(context),
            _ => None,
        }
    }
}
