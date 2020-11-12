#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ImageDimensions<T> {
    pub width: T,
    pub height: T,
    pub depth: T,
    pub channels: T,
}

impl<T: num_traits::identities::One> ImageDimensions<T> {
    pub fn new(width: T, height: T, channels: T) -> Self {
        Self {
            width,
            height,
            depth: num_traits::One::one(),
            channels,
        }
    }

    pub fn new_1d(width: T, channels: T) -> Self {
        Self {
            width,
            height: num_traits::One::one(),
            depth: num_traits::One::one(),
            channels,
        }
    }

    pub fn new_3d(width: T, height: T, depth: T, channels: T) -> Self {
        Self {
            width,
            height,
            depth,
            channels,
        }
    }

    pub fn into_nd_array_dim(self) -> (T, T, T, T) {
        <Self as Into<(T, T, T, T)>>::into(self)
    }
}

impl<T> From<(T, T, T, T)> for ImageDimensions<T> {
    fn from(dim: (T, T, T, T)) -> Self {
        Self {
            depth: dim.0,
            height: dim.1,
            width: dim.2,
            channels: dim.3,
        }
    }
}

impl<T> Into<(T, T, T, T)> for ImageDimensions<T> {
    fn into(self) -> (T, T, T, T) {
        (self.depth, self.height, self.width, self.channels)
    }
}

pub type ImageDim = ImageDimensions<usize>;

#[cfg(feature = "gpu-core")]
mod gpu {
    use super::ImageDim;
    use crate::image::ImageDataType;

    use tinygl::gl;

    pub trait ImageDimGpuExt {
        fn internal_format(&self, element_type: ImageDataType) -> Option<i32>;

        fn unsized_format(&self) -> Option<u32>;

        fn into_cgmath(&self) -> cgmath::Vector3<u32>;
    }

    impl ImageDimGpuExt for ImageDim {
        fn internal_format(&self, element_type: ImageDataType) -> Option<i32> {
            match element_type {
                ImageDataType::UInt8 => match self.channels {
                    1 => Some(gl::R8 as i32),
                    2 => Some(gl::RG8 as i32),
                    3 => Some(gl::RGB8 as i32),
                    4 => Some(gl::RGBA8 as i32),
                    _ => None,
                },
                ImageDataType::Float32 => match self.channels {
                    1 => Some(gl::R32F as i32),
                    2 => Some(gl::RG32F as i32),
                    3 => Some(gl::RGB32F as i32),
                    4 => Some(gl::RGBA32F as i32),
                    _ => None,
                },
            }
        }
        fn unsized_format(&self) -> Option<u32> {
            match self.channels {
                1 => Some(gl::RED),
                2 => Some(gl::RG),
                3 => Some(gl::RGB),
                4 => Some(gl::RGBA),
                _ => None,
            }
        }

        fn into_cgmath(&self) -> cgmath::Vector3<u32> {
            cgmath::vec3(self.width as u32, self.height as u32, self.depth as u32)
        }
    }
}

#[cfg(feature = "gpu-core")]
pub use gpu::*;
