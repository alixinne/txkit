use std::any::Any;

use txkit_core::{
    context::Context,
    image::{Image, ImageDataType, ImageDim, MappedImageData, MappedImageDataMut},
    method::{Method, MethodRegistry},
    Error,
};

pub mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

mod api;

/// Wrapped method for FFI
pub struct MethodBox {
    method: Box<dyn Method>,
}

/// Compute an image using the given method
///
/// # Parameters
///
/// * `ctx`: context to use for computing the image
/// * `method`: texturing method
/// * `tgt`: target image to be computed
/// * `params`: pointer to the parameter structure for this method
/// * `params_size`: size of the parameter structure
///
/// # Returns
///
/// TxKit_SUCCESS if no error occurred, else a non-zero code.
#[no_mangle]
pub unsafe extern "C" fn txkit_method_compute(
    ctx: &mut Context,
    method: &mut MethodBox,
    tgt: &mut Image,
    params: *const std::ffi::c_void,
    params_size: usize,
) -> i32 {
    let params_slice;
    let params: Option<&dyn Any> = if params == std::ptr::null() {
        None
    } else {
        params_slice = std::slice::from_raw_parts(params as *const u8, params_size);
        Some(&params_slice)
    };

    crate::api::wrap_result_code(|| method.method.compute(ctx, tgt, params))
}

/// Destroy a method
///
/// # Parameters
///
/// * `method`: method to destroy
#[no_mangle]
pub unsafe extern "C" fn txkit_method_destroy(method: *mut MethodBox) {
    std::mem::drop(Box::from_raw(method))
}

/// Wrapped registry for FFI
pub struct RegistryBox {
    registry: Box<MethodRegistry>,
}

/// Create a new registry with txkit built-in methods registered
///
/// # Returns
///
/// Pointer to the allocated registry.
#[no_mangle]
pub extern "C" fn txkit_registry_new_builtin() -> *mut RegistryBox {
    crate::api::wrap(|| {
        Box::into_raw(Box::new(RegistryBox {
            registry: Box::new(txkit_builtin::methods::new_registry()),
        }))
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Create a new method by name
///
/// # Parameters
///
/// * `registry`: registry of methods to build from
/// * `method_name`: name of the method to create
///
/// # Returns
///
/// Null pointer if an error occurred creating the method, otherwise pointer to the allocated
/// method.
#[no_mangle]
pub extern "C" fn txkit_method_new(
    registry: &RegistryBox,
    method_name: *const libc::c_char,
) -> *mut MethodBox {
    crate::api::wrap_result(|| {
        if method_name == std::ptr::null() {
            Err(Error::InvalidMethodName)
        } else {
            match unsafe { std::ffi::CStr::from_ptr(method_name as *const _) }.to_str() {
                Ok(method) => {
                    if let Some(method) = registry.registry.build(method) {
                        Ok(Box::into_raw(Box::new(MethodBox { method })))
                    } else {
                        Err(Error::MethodNotFound)
                    }
                }
                Err(_) => Err(Error::InvalidMethodName),
            }
        }
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroy a registry
///
/// # Parameters
///
/// * `registry`: registry to destroy
#[no_mangle]
pub unsafe extern "C" fn txkit_registry_destroy(registry: *mut RegistryBox) {
    std::mem::drop(Box::from_raw(registry))
}

/// Create a new image for CPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_cpu(dim: ImageDim, element_type: ImageDataType) -> *mut Image {
    Box::into_raw(Box::new(Image::new_cpu(dim, element_type)))
}

/// Create a new 1D image for GPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_gpu_1d(
    dim: ImageDim,
    element_type: ImageDataType,
    context: &Context,
) -> *mut Image {
    crate::api::wrap_result(|| {
        Image::new_gpu_1d(dim, element_type, context)
            .map(Box::new)
            .map(Box::into_raw)
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Create a new 2D image for GPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_gpu_2d(
    dim: ImageDim,
    element_type: ImageDataType,
    context: &Context,
) -> *mut Image {
    crate::api::wrap_result(|| {
        Image::new_gpu_2d(dim, element_type, context)
            .map(Box::new)
            .map(Box::into_raw)
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Create a new 3D image for GPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_gpu_3d(
    dim: ImageDim,
    element_type: ImageDataType,
    context: &Context,
) -> *mut Image {
    crate::api::wrap_result(|| {
        Image::new_gpu_3d(dim, element_type, context)
            .map(Box::new)
            .map(Box::into_raw)
    })
    .unwrap_or(std::ptr::null_mut())
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

/// Return the dimensions of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_dim(image: &Image) -> ImageDim {
    image.dim()
}

/// Sync the host representation of the image with its device counterpart
///
/// # Parameters
///
/// * `image`: image to sync
#[no_mangle]
pub extern "C" fn txkit_image_sync(image: &mut Image) -> i32 {
    crate::api::wrap_result_code(|| image.sync())
}

/// Wrapped read-only mapping for FFI
pub struct MappedImageDataReadBox {
    ptr: Box<dyn MappedImageData>,
}

/// Map the image pixels for read access. The image must be unmapped after being used.
///
/// # Parameters
///
/// * `image`: image to map for read access
#[no_mangle]
pub extern "C" fn txkit_image_map_read(image: &'static Image) -> *mut MappedImageDataReadBox {
    crate::api::wrap_result(|| {
        image
            .data()
            .map(|bx| Box::into_raw(Box::new(MappedImageDataReadBox { ptr: bx })))
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `read_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_read_data_u8(read_map: &MappedImageDataReadBox) -> *const u8 {
    read_map
        .ptr
        .as_u8_nd_array()
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `read_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_read_data_f32(read_map: &MappedImageDataReadBox) -> *const f32 {
    read_map
        .ptr
        .as_f32_nd_array()
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Unmap a mapped image.
///
/// # Parameters
///
/// * `read_map`: mapped image object
#[no_mangle]
pub unsafe extern "C" fn txkit_image_unmap_read(read_map: *mut MappedImageDataReadBox) {
    std::mem::drop(Box::from_raw(read_map))
}

/// Wrapped read-write mapping for FFI
pub struct MappedImageDataWriteBox {
    ptr: Box<dyn MappedImageDataMut>,
}

/// Map the image pixels for write access. The image must be unmapped after being used.
///
/// # Parameters
///
/// * `image`: image to map for write access
#[no_mangle]
pub extern "C" fn txkit_image_map_write(image: &'static mut Image) -> *mut MappedImageDataWriteBox {
    crate::api::wrap_result(move || {
        image
            .data_mut()
            .map(|bx| Box::into_raw(Box::new(MappedImageDataWriteBox { ptr: bx })))
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `write_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_write_data_u8(
    write_map: &mut MappedImageDataWriteBox,
) -> *mut u8 {
    write_map
        .ptr
        .as_u8_nd_array_mut()
        .map(|mut ptr| ptr.as_mut_ptr())
        .unwrap_or(std::ptr::null_mut())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `write_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_write_data_f32(
    write_map: &mut MappedImageDataWriteBox,
) -> *mut f32 {
    write_map
        .ptr
        .as_f32_nd_array_mut()
        .map(|mut ptr| ptr.as_mut_ptr())
        .unwrap_or(std::ptr::null_mut())
}

/// Unmap a mapped image.
///
/// # Parameters
///
/// * `write_map`: mapped image object
#[no_mangle]
pub unsafe extern "C" fn txkit_image_unmap_write(write_map: *mut MappedImageDataWriteBox) {
    std::mem::drop(Box::from_raw(write_map))
}

/// Create a new CPU context
///
/// # Returns
///
/// Pointer to the created context, or null if the creation failed.
#[no_mangle]
pub extern "C" fn txkit_context_new_cpu() -> *mut Context {
    crate::api::wrap_result(|| Context::new_cpu().map(Box::new).map(Box::into_raw))
        .unwrap_or(std::ptr::null_mut())
}

/// Create a new GPU context
///
/// # Returns
///
/// Pointer to the created context, or null if the creation failed.
#[no_mangle]
pub extern "C" fn txkit_context_new_gpu() -> *mut Context {
    crate::api::wrap_result(|| Context::new_gpu().map(Box::new).map(Box::into_raw))
        .unwrap_or(std::ptr::null_mut())
}

/// Destroy a context
///
/// # Parameters
///
/// * `ctx`: context to destroy
#[no_mangle]
pub unsafe extern "C" fn txkit_context_destroy(ctx: *mut Context) {
    std::mem::drop(Box::from_raw(ctx))
}
