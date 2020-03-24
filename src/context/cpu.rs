/// txkit internal context for CPU computations
pub struct CpuContext {
    pub(crate) thread_pool: rayon::ThreadPool,
}

impl CpuContext {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            thread_pool: rayon::ThreadPoolBuilder::new()
                .build()
                .map_err(|e| e.to_string())?,
        })
    }
}
