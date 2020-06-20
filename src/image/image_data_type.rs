/// Type of elements in an image
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageDataType {
    /// Unsigned bytes (8 bits)
    UInt8,
    /// Single-precision floating point (32 bits)
    Float32,
}

impl ImageDataType {
    pub fn byte_size(&self) -> usize {
        match self {
            Self::UInt8 => std::mem::size_of::<u8>(),
            Self::Float32 => std::mem::size_of::<f32>(),
        }
    }
}

#[cfg(feature = "gpu-core")]
mod gpu {
    use super::ImageDataType;
    use tinygl::gl;

    pub trait ImageDataTypeGpuExt {
        fn format_type(&self) -> u32;
    }

    impl ImageDataTypeGpuExt for ImageDataType {
        fn format_type(&self) -> u32 {
            match self {
                ImageDataType::UInt8 => gl::UNSIGNED_BYTE,
                ImageDataType::Float32 => gl::FLOAT,
            }
        }
    }
}

#[cfg(feature = "gpu-core")]
pub use gpu::*;
