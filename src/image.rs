use ndarray::Array4;
use strum_macros::EnumDiscriminants;

/// Image that can be sent accross for FFI
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ImageDataType))]
#[strum_discriminants(repr(u32))]
pub enum Image {
    /// Unsigned bytes (8 bits) image
    UInt8(Array4<u8>),
    /// Single-precision floating point (32 bits) image
    Float32(Array4<f32>),
}

macro_rules! each_type {
    ($on:ident, $with:pat => $e:expr) => {
        match $on {
            Self::UInt8($with) => $e,
            Self::Float32($with) => $e,
        }
    }
}

impl Image {
    pub fn new_u8(width: usize, height: usize, channels: usize) -> Self {
        Self::UInt8(Array4::<_>::zeros((width, height, 1, channels)))
    }

    pub fn new_f32(width: usize, height: usize, channels: usize) -> Self {
        Self::Float32(Array4::<_>::zeros((width, height, 1, channels)))
    }

    pub fn element_type(&self) -> ImageDataType {
        self.into()
    }

    pub fn dim(&self) -> (usize, usize, usize, usize) {
        each_type!(self, data => data.dim())
    }

    pub fn width(&self) -> usize {
        self.dim().0
    }

    pub fn height(&self) -> usize {
        self.dim().1
    }

    pub fn depth(&self) -> usize {
        self.dim().2
    }

    pub fn channels(&self) -> usize {
        self.dim().3
    }

    pub(crate) unsafe fn as_ptr(&self) -> *const std::ffi::c_void {
        each_type!(self, data => data.as_ptr() as *const std::ffi::c_void)
    }
}

/// Create a new unsigned byte image
///
/// # Parameters
///
/// * `width`: width of the image
/// * `height`: height of the image
/// * `channels`: number of channels in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_u8(width: usize, height: usize, channels: usize) -> *mut Image {
    Box::into_raw(Box::new(Image::new_u8(width, height, channels)))
}

/// Create a new unsigned byte image
///
/// # Parameters
///
/// * `width`: width of the image
/// * `height`: height of the image
/// * `channels`: number of channels in the image
///
/// # Returns
///
#[no_mangle]
pub extern "C" fn txkit_image_new_f32(width: usize, height: usize, channels: usize) -> *mut Image {
    Box::into_raw(Box::new(Image::new_f32(width, height, channels)))
}

/// Destroy an image
///
/// # Parameters
///
/// * `image`: image to destroy
#[no_mangle]
pub unsafe extern "C" fn txkit_image_destroy(image: *mut Image) {
    std::mem::drop(Box::from_raw(image))
}

/// Return the element type of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_element_type(image: &Image) -> ImageDataType {
    image.element_type()
}

/// Return the width (X size) of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_width(image: &Image) -> u32 {
    image.width() as u32
}

/// Return the height (Y size) of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_height(image: &Image) -> u32 {
    image.height() as u32
}

/// Return the depth (Z size) of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_depth(image: &Image) -> u32 {
    image.depth() as u32
}

/// Return the number of channels of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_channels(image: &Image) -> u32 {
    image.channels() as u32
}

/// Return a pointer to the image data
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub unsafe extern "C" fn txkit_image_data(image: &Image) -> *const std::ffi::c_void {
    image.as_ptr()
}
