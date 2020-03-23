use ndarray::Array4;

/// Image that can be sent accross for FFI
#[derive(Debug, Clone)]
pub enum Image {
    UInt8(Array4<u8>),
    Float32(Array4<f32>),
}

/// Type of elements in an image
#[repr(u32)]
pub enum ImageDataType {
    /// Unsigned bytes (8 bits)
    UInt8,
    /// Single-precision floating point (32 bits)
    Float32,
}

impl Image {
    pub fn new_u8(width: usize, height: usize, channels: usize) -> Self {
        Self::UInt8(Array4::<_>::zeros((width, height, 1, channels)))
    }

    pub fn new_f32(width: usize, height: usize, channels: usize) -> Self {
        Self::Float32(Array4::<_>::zeros((width, height, 1, channels)))
    }

    pub fn element_type(&self) -> ImageDataType {
        match self {
            Self::UInt8(_) => ImageDataType::UInt8,
            Self::Float32(_) => ImageDataType::Float32,
        }
    }

    pub fn dim(&self) -> (usize, usize, usize, usize) {
        match self {
            Self::UInt8(data) => data.dim(),
            Self::Float32(data) => data.dim(),
        }
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
        match self {
            Self::UInt8(data) => data.as_ptr() as *const std::ffi::c_void,
            Self::Float32(data) => data.as_ptr() as *const std::ffi::c_void,
        }
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
