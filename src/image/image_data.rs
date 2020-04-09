use failure::Fail;
use ndarray::{ArrayView4, ArrayViewMut4};

use super::{ImageDataType, ImageDim};

pub trait ImageDataBase {
    /// Get the dimensions of the stored image
    fn dim(&self) -> ImageDim;

    /// Get the native type of elements in this image
    fn element_type(&self) -> ImageDataType;

    /// Sync the contents of the (mappable) host data with device data.
    /// Required for GPU backends. May be asynchronous.
    fn sync(&mut self) -> Result<(), String> {
        Ok(())
    }

    #[cfg(feature = "gpu")]
    /// Get the image data as a GpuImageData reference if possible
    fn as_gpu_image(&self) -> Option<&super::gpu::GpuImageData> {
        None
    }

    #[cfg(feature = "gpu")]
    /// Get the image data as a mutable GpuImageData reference if possible
    fn as_gpu_image_mut(&mut self) -> Option<&mut super::gpu::GpuImageData> {
        None
    }
}

pub trait MappedImageData {
    /// Get the image as an f32 nd-array
    fn as_f32_nd_array(&self) -> Option<ArrayView4<f32>> {
        None
    }

    /// Get the image as an u8 nd-array
    fn as_u8_nd_array(&self) -> Option<ArrayView4<u8>> {
        None
    }
}

pub trait MappedImageDataMut {
    /// Get the image as a mutable f32 nd-array
    fn as_f32_nd_array_mut(&mut self) -> Option<ArrayViewMut4<f32>> {
        None
    }

    /// Get the image as a mutable u8 nd-array
    fn as_u8_nd_array_mut(&mut self) -> Option<ArrayViewMut4<u8>> {
        None
    }
}

#[derive(Debug, Fail, Clone, Eq, PartialEq)]
pub enum ImageDataError {
    #[fail(display = "unknown mapping error")]
    UnknownMapping,
    #[fail(display = "image needs to be synced before being mapped")]
    Unsynced,
    #[fail(display = "mapping the image failed in the backend, check logs for details")]
    MappingFailed,
}

pub trait ImageData: ImageDataBase {
    fn data(&self) -> Result<Box<dyn MappedImageData + '_>, ImageDataError>;
    fn data_mut(&mut self) -> Result<Box<dyn MappedImageDataMut + '_>, ImageDataError>;
}
