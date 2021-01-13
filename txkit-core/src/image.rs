mod cpu;
#[cfg(feature = "gpu-core")]
pub mod gpu;

mod image_data;
pub use image_data::*;

mod image_data_type;
pub use image_data_type::*;

mod image_dimensions;
pub use image_dimensions::*;

mod into_element_type;
pub use into_element_type::*;

pub mod prelude;

use thiserror::Error;

/// Image that can be sent accross for FFI
pub struct Image {
    data: Box<dyn ImageData>,
}

impl std::ops::Deref for Image {
    type Target = dyn ImageData;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.data.as_ref()
    }
}

impl std::ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        self.data.as_mut()
    }
}

#[derive(Debug, Error)]
pub enum ImageCreationError {
    #[error("the context cannot create this type of images")]
    ContextNotSupported,
    #[cfg(feature = "gpu-core")]
    #[error("failed to create the image: {0}")]
    ImageCreationFailed(#[from] tinygl::Error),
    #[error("unsupported number of channels: {0} (expected <= 4)")]
    InvalidChannelCount(usize),
    #[error("invalid image dimensions for the requested dimension")]
    InvalidImageSize,
}

impl Image {
    pub fn new_cpu(dim: ImageDim, element_type: ImageDataType) -> Self {
        Self {
            data: match element_type {
                ImageDataType::UInt8 => Box::new(cpu::UInt8ImageData::new(dim)),
                ImageDataType::Float32 => Box::new(cpu::FloatImageData::new(dim)),
            },
        }
    }

    #[cfg(feature = "gpu-core")]
    pub fn new_gpu_1d(
        dim: ImageDim,
        element_type: ImageDataType,
        context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        context
            .gpu()
            .ok_or(ImageCreationError::ContextNotSupported)
            .and_then(|gpu_context| {
                Ok(Self {
                    data: Box::new(gpu::GpuImageData::new_1d(
                        &gpu_context.gl,
                        dim,
                        element_type,
                    )?),
                })
            })
    }

    #[cfg(feature = "gpu-core")]
    pub fn new_gpu_2d(
        dim: ImageDim,
        element_type: ImageDataType,
        context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        context
            .gpu()
            .ok_or(ImageCreationError::ContextNotSupported)
            .and_then(|gpu_context| {
                Ok(Self {
                    data: Box::new(gpu::GpuImageData::new_2d(
                        &gpu_context.gl,
                        dim,
                        element_type,
                    )?),
                })
            })
    }

    #[cfg(feature = "gpu-core")]
    pub fn new_gpu_3d(
        dim: ImageDim,
        element_type: ImageDataType,
        context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        context
            .gpu()
            .ok_or(ImageCreationError::ContextNotSupported)
            .and_then(|gpu_context| {
                Ok(Self {
                    data: Box::new(gpu::GpuImageData::new_3d(
                        &gpu_context.gl,
                        dim,
                        element_type,
                    )?),
                })
            })
    }

    #[cfg(not(feature = "gpu-core"))]
    pub fn new_gpu_1d(
        _dim: ImageDim,
        _element_type: ImageDataType,
        _context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        Err(ImageCreationError::ContextNotSupported)
    }

    #[cfg(not(feature = "gpu-core"))]
    pub fn new_gpu_2d(
        _dim: ImageDim,
        _element_type: ImageDataType,
        _context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        Err(ImageCreationError::ContextNotSupported)
    }

    #[cfg(not(feature = "gpu-core"))]
    pub fn new_gpu_3d(
        _dim: ImageDim,
        _element_type: ImageDataType,
        _context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        Err(ImageCreationError::ContextNotSupported)
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dim = self.dim();
        write!(
            f,
            "[{}w x {}h x {}d x {}c image]",
            dim.width, dim.height, dim.depth, dim.channels
        )
    }
}
