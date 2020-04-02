use ndarray::{Array4, ArrayView4, ArrayViewMut4};

use super::{
    ImageData, ImageDataBase, ImageDataError, ImageDataType, ImageDim, MappedImageData,
    MappedImageDataMut,
};

pub struct NdArrayImageData<T> {
    data: Array4<T>,
}

impl<T: num_traits::identities::Zero + num_traits::identities::Zero + std::clone::Clone>
    NdArrayImageData<T>
{
    pub fn new(dim: ImageDim) -> Self {
        Self {
            data: Array4::<T>::zeros(Into::<(usize, usize, usize, usize)>::into(dim)),
        }
    }
}

pub trait IntoElementType {
    fn into_element_type() -> ImageDataType;
    fn into_f32(&self) -> f32;
}

impl IntoElementType for f32 {
    fn into_element_type() -> ImageDataType {
        ImageDataType::Float32
    }

    fn into_f32(&self) -> f32 {
        *self
    }
}

impl IntoElementType for u8 {
    fn into_element_type() -> ImageDataType {
        ImageDataType::UInt8
    }

    fn into_f32(&self) -> f32 {
        *self as f32 / 255.0f32
    }
}

impl<T: IntoElementType> ImageDataBase for NdArrayImageData<T> {
    fn dim(&self) -> ImageDim {
        self.data.dim().into()
    }

    fn element_type(&self) -> ImageDataType {
        T::into_element_type()
    }
}

struct MappedNdArray<T> {
    tgt: T,
}

macro_rules! _mapped_image_data {
    ($t:ty, $($ts:ty),+) => {
        _mapped_image_data!($t);
        _mapped_image_data!($($ts),+);
    };

    ($t:ty) => {
        impl MappedImageData for MappedNdArray<&NdArrayImageData<$t>> {
            paste::item! {
                fn [<as_ $t _nd_array>](&self) -> Option<ArrayView4<$t>> {
                    Some(ArrayView4::from(&self.tgt.data))
                }
            }
        }

        impl MappedImageDataMut for MappedNdArray<&mut NdArrayImageData<$t>> {
            paste::item! {
                fn [<as_ $t _nd_array_mut>](&mut self) -> Option<ArrayViewMut4<$t>> {
                    Some(ArrayViewMut4::from(&mut self.tgt.data))
                }
            }
        }
    }
}

macro_rules! _image_data {
    ($t:ty, $($ts:ty),+) => {
        _image_data!($t);
        _image_data!($($ts),+);
    };

    ($t:ty) => {
        impl ImageData for NdArrayImageData<$t> {
            fn data(&self) -> Result<Box<dyn MappedImageData + '_>, ImageDataError> {
                Ok(Box::new(MappedNdArray { tgt: self }))
            }

            fn data_mut(&mut self) -> Result<Box<dyn MappedImageDataMut + '_>, ImageDataError> {
                Ok(Box::new(MappedNdArray { tgt: self }))
            }
        }
    };
}

macro_rules! _type_def {
    ($t:ty => $n:ident, $($ts:ty => $ns:ident),+) => {
        _type_def!($t => $n);
        _type_def!($($ts => $ns),+);
    };

    ($t:ty => $n:ident) => {
        paste::item! { pub type [<$n ImageData>] = NdArrayImageData<$t>; }
    };
}

macro_rules! impl_for_types {
    ($($ts:ty => $n:ident),+) => {
        _mapped_image_data!($($ts),+);
        _image_data!($($ts),+);
        _type_def!($($ts => $n),+);
    };
}

impl_for_types!(u8 => UInt8, f32 => Float);
