//! GPU Procedural texturing method types

use crate::context::GpuContext;
use crate::image::gpu::GpuImageData;
use crate::Result;

/// Represents a GPU procedural texturing method
pub trait GpuMethod {
    /// Texture generation parameters
    type Params;

    /// Compute one frame of this method
    ///
    /// # Parameters
    ///
    /// * `ctx`: GPU context to perform computations in
    /// * `tgt`: frame to fill with computation results
    /// * `params`: parameters of the frame to compute
    fn compute_gpu(
        &mut self,
        ctx: &mut GpuContext,
        tgt: &mut GpuImageData,
        params: &Self::Params,
    ) -> Result<()>;
}

/// Represents a set of parameters for a given method
///
/// # Type parameters
///
/// * `gl`: OpenGL context
/// * `P`: type of the program to set the values on
pub trait GpuMethodParams<P> {
    fn apply(&self, gl: &tinygl::Context, p: &P);
}
