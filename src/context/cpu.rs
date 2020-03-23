/// txkit internal context for CPU computations
pub struct CpuContext {}

impl CpuContext {
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }
}
