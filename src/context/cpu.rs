use crate::Result;

/// txkit internal context for CPU computations
pub struct CpuContext {
    pub(crate) thread_pool: rayon::ThreadPool,
}

impl CpuContext {
    pub fn new() -> Result<Self> {
        Ok(Self {
            thread_pool: rayon::ThreadPoolBuilder::new().build()?,
        })
    }
}
