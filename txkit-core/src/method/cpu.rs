//! CPU Procedural texturing method types

use crate::context::CpuContext;
use crate::image::Image;
use crate::Result;

/// Represents a CPU procedural texturing method
pub trait CpuMethod {
    /// Texture generation parameters
    type Params;

    /// Compute one frame of this method
    ///
    /// # Parameters
    ///
    /// * `ctx`: CPU context to perform computations in
    /// * `tgt`: frame to fill with computation results
    /// * `params`: parameters of the frame to compute
    fn compute_cpu(
        &mut self,
        ctx: &mut CpuContext,
        tgt: &mut Image,
        params: &Self::Params,
    ) -> Result<()>;
}
