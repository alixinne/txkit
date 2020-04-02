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
