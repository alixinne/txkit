use super::ImageDataType;

pub trait IntoElementType {
    fn into_element_type() -> ImageDataType;
    fn into_u8(&self) -> u8;
    fn into_f32(&self) -> f32;
}

impl IntoElementType for f32 {
    fn into_element_type() -> ImageDataType {
        ImageDataType::Float32
    }

    fn into_u8(&self) -> u8 {
        (*self * 255.0f32).min(255.0f32).max(0.0f32) as u8
    }

    fn into_f32(&self) -> f32 {
        *self
    }
}

impl IntoElementType for u8 {
    fn into_element_type() -> ImageDataType {
        ImageDataType::UInt8
    }

    fn into_u8(&self) -> u8 {
        *self
    }

    fn into_f32(&self) -> f32 {
        *self as f32 / 255.0f32
    }
}
