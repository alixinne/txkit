use std::cell::RefCell;
use std::rc::Rc;

use tinygl::prelude::*;
use tinygl::wrappers::{Buffer, GlRefHandle, Texture};

use super::*;

pub struct GpuImageData {
    gl: Rc<tinygl::Context>,
    pub(crate) texture: Texture,
    pub(crate) buffer: Buffer,
    pub(crate) element_type: ImageDataType,
    pub(crate) dim: ImageDim,

    target: u32,
    transfer_sync: RefCell<Option<<tinygl::glow::Context as tinygl::glow::HasContext>::Fence>>,
}

impl GpuImageData {
    fn new_nd(
        gl: &Rc<tinygl::Context>,
        dim: ImageDim,
        element_type: ImageDataType,
        target: u32,
        allocator: impl Fn(ImageDim, ImageDataType) -> Result<(), String>,
    ) -> Result<Self, String> {
        let texture = GlRefHandle::new(&*gl, Texture::new(gl)?);
        texture.bind(gl, target);

        // Allocation result
        let res = unsafe {
            gl.tex_parameter_i32(
                target,
                tinygl::gl::TEXTURE_MIN_FILTER,
                tinygl::gl::NEAREST as i32,
            );

            gl.tex_parameter_i32(
                target,
                tinygl::gl::TEXTURE_MAG_FILTER,
                tinygl::gl::NEAREST as i32,
            );

            allocator(dim, element_type)?;

            // Check that the allocation succeeded
            gl.check_last_error().map_err(|e| e.to_string())
        };

        // Unbind texture after allocation
        unsafe {
            gl.bind_texture(target, None);
        }

        let buffer = GlRefHandle::new(&*gl, Buffer::new(gl)?);

        res.map(|()| Self {
            gl: gl.clone(),
            texture: texture.into_inner(),
            buffer: buffer.into_inner(),
            element_type,
            dim,
            transfer_sync: RefCell::new(None),
            target,
        })
    }

    pub fn new_1d(
        gl: &Rc<tinygl::Context>,
        dim: ImageDim,
        element_type: ImageDataType,
    ) -> Result<Self, String> {
        if dim.height > 1 {
            return Err("invalid height for 1D image".to_owned());
        }

        if dim.depth > 1 {
            return Err("invalid depth for 1D image".to_owned());
        }

        Self::new_nd(
            gl,
            dim,
            element_type,
            tinygl::gl::TEXTURE_1D,
            |dim, element_type| {
                unsafe {
                    gl.tex_image_1d(
                        tinygl::gl::TEXTURE_1D,
                        0,
                        dim.internal_format(element_type)?,
                        dim.width as i32,
                        0,
                        dim.unsized_format()?,
                        element_type.format_type(),
                        None,
                    );
                }

                Ok(())
            },
        )
    }

    pub fn new_2d(
        gl: &Rc<tinygl::Context>,
        dim: ImageDim,
        element_type: ImageDataType,
    ) -> Result<Self, String> {
        if dim.depth > 1 {
            return Err("invalid depth for 2D image".to_owned());
        }

        Self::new_nd(
            gl,
            dim,
            element_type,
            tinygl::gl::TEXTURE_2D,
            |dim, element_type| {
                unsafe {
                    gl.tex_image_2d(
                        tinygl::gl::TEXTURE_2D,
                        0,
                        dim.internal_format(element_type)?,
                        dim.width as i32,
                        dim.height as i32,
                        0,
                        dim.unsized_format()?,
                        element_type.format_type(),
                        None,
                    );
                }

                Ok(())
            },
        )
    }

    pub fn new_3d(
        gl: &Rc<tinygl::Context>,
        dim: ImageDim,
        element_type: ImageDataType,
    ) -> Result<Self, String> {
        Self::new_nd(
            gl,
            dim,
            element_type,
            tinygl::gl::TEXTURE_3D,
            |dim, element_type| {
                unsafe {
                    gl.tex_image_3d(
                        tinygl::gl::TEXTURE_3D,
                        0,
                        dim.internal_format(element_type)?,
                        dim.width as i32,
                        dim.height as i32,
                        dim.depth as i32,
                        0,
                        dim.unsized_format()?,
                        element_type.format_type(),
                        None,
                    );
                }

                Ok(())
            },
        )
    }

    pub fn byte_size(&self) -> usize {
        self.element_type.byte_size()
            * self.dim.width
            * self.dim.height
            * self.dim.depth
            * self.dim.channels
    }

    pub fn start_download(&mut self) -> Result<(), String> {
        unsafe {
            self.buffer.bind(&*self.gl, tinygl::gl::PIXEL_PACK_BUFFER);
            self.gl.buffer_data_size(
                tinygl::gl::PIXEL_PACK_BUFFER,
                self.byte_size() as i32,
                tinygl::gl::DYNAMIC_READ,
            );

            self.gl.check_last_error().map_err(|e| e.to_string())?;

            self.texture.bind(&*self.gl, self.target);
            self.gl.get_tex_image_pixel_buffer_offset(
                self.target,
                0,
                self.dim.unsized_format()?,
                self.element_type.format_type(),
                0,
            );

            self.gl.bind_buffer(tinygl::gl::PIXEL_PACK_BUFFER, None);

            *self.transfer_sync.borrow_mut() = Some(
                self.gl
                    .fence_sync(tinygl::gl::SYNC_GPU_COMMANDS_COMPLETE, 0)?,
            );

            Ok(())
        }
    }

    unsafe fn map_buffer(&self, usage: u32) -> *mut u8 {
        if let Some(fence_sync) = self.transfer_sync.borrow_mut().take() {
            loop {
                match self.gl.client_wait_sync(
                    fence_sync,
                    tinygl::gl::SYNC_FLUSH_COMMANDS_BIT,
                    10_000_000,
                ) {
                    tinygl::gl::ALREADY_SIGNALED | tinygl::gl::CONDITION_SATISFIED => {
                        break;
                    }
                    tinygl::gl::TIMEOUT_EXPIRED => {
                        // keep waiting
                    }
                    tinygl::gl::WAIT_FAILED => {
                        // TODO: What to do here?
                        break;
                    }
                    _ => {}
                }
            }
        }

        self.buffer.bind(&*self.gl, tinygl::gl::PIXEL_PACK_BUFFER);

        let ptr = self.gl.map_buffer_range(
            tinygl::gl::PIXEL_PACK_BUFFER,
            0,
            self.byte_size() as i32,
            usage,
        );

        self.gl.bind_buffer(tinygl::gl::PIXEL_PACK_BUFFER, None);

        ptr
    }

    unsafe fn unmap_buffer(&self) {
        self.buffer.bind(&*self.gl, tinygl::gl::PIXEL_PACK_BUFFER);
        self.gl.unmap_buffer(tinygl::gl::PIXEL_PACK_BUFFER);
        self.gl.bind_buffer(tinygl::gl::PIXEL_PACK_BUFFER, None);
    }
}

impl Drop for GpuImageData {
    fn drop(&mut self) {
        use tinygl::wrappers::GlDrop;

        self.texture.drop(&*self.gl);
        self.buffer.drop(&*self.gl);
    }
}

impl ImageDataBase for GpuImageData {
    fn dim(&self) -> ImageDim {
        self.dim
    }
    fn element_type(&self) -> ImageDataType {
        self.element_type
    }
    fn as_gpu_image(&self) -> Option<&GpuImageData> {
        Some(self)
    }
    fn as_gpu_image_mut(&mut self) -> Option<&mut GpuImageData> {
        Some(self)
    }
}

struct MappedGpuImage<'t> {
    tgt: &'t GpuImageData,
    mapped_ptr: *const u8,
}

struct MappedGpuImageMut<'t> {
    tgt: &'t mut GpuImageData,
    mapped_ptr: *mut u8,
}

impl<'t> MappedGpuImage<'t> {
    fn map(tgt: &'t GpuImageData) -> Result<Self, ImageDataError> {
        let mapped_ptr = unsafe { tgt.map_buffer(tinygl::gl::MAP_READ_BIT) };

        Ok(Self { tgt, mapped_ptr })
    }
}

impl Drop for MappedGpuImage<'_> {
    fn drop(&mut self) {
        unsafe {
            self.tgt.unmap_buffer();
        }
    }
}

impl<'t> MappedGpuImageMut<'t> {
    fn map(tgt: &'t mut GpuImageData) -> Result<Self, ImageDataError> {
        let mapped_ptr =
            unsafe { tgt.map_buffer(tinygl::gl::MAP_READ_BIT | tinygl::gl::MAP_WRITE_BIT) };

        Ok(Self { tgt, mapped_ptr })
    }
}

impl Drop for MappedGpuImageMut<'_> {
    fn drop(&mut self) {
        unsafe {
            self.tgt.unmap_buffer();
        }
    }
}

impl MappedImageData for MappedGpuImage<'_> {
    fn as_f32_nd_array(&self) -> Option<ndarray::ArrayView4<f32>> {
        if let ImageDataType::Float32 = self.tgt.element_type {
            unsafe {
                Some(ndarray::ArrayView4::from_shape_ptr(
                    self.tgt.dim.into_nd_array_dim(),
                    std::mem::transmute::<*const u8, *const f32>(self.mapped_ptr),
                ))
            }
        } else {
            None
        }
    }

    fn as_u8_nd_array(&self) -> Option<ndarray::ArrayView4<u8>> {
        if let ImageDataType::UInt8 = self.tgt.element_type {
            unsafe {
                Some(ndarray::ArrayView4::from_shape_ptr(
                    self.tgt.dim.into_nd_array_dim(),
                    self.mapped_ptr,
                ))
            }
        } else {
            None
        }
    }
}

impl MappedImageDataMut for MappedGpuImageMut<'_> {
    fn as_f32_nd_array_mut(&mut self) -> Option<ndarray::ArrayViewMut4<f32>> {
        if let ImageDataType::Float32 = self.tgt.element_type {
            unsafe {
                Some(ndarray::ArrayViewMut4::from_shape_ptr(
                    self.tgt.dim.into_nd_array_dim(),
                    std::mem::transmute::<*mut u8, *mut f32>(self.mapped_ptr),
                ))
            }
        } else {
            None
        }
    }

    fn as_u8_nd_array_mut(&mut self) -> Option<ndarray::ArrayViewMut4<u8>> {
        if let ImageDataType::UInt8 = self.tgt.element_type {
            unsafe {
                Some(ndarray::ArrayViewMut4::from_shape_ptr(
                    self.tgt.dim.into_nd_array_dim(),
                    self.mapped_ptr,
                ))
            }
        } else {
            None
        }
    }
}

impl ImageData for GpuImageData {
    fn data(&self) -> Result<Box<dyn MappedImageData + '_>, ImageDataError> {
        Ok(Box::new(MappedGpuImage::map(self)?))
    }

    fn data_mut(&mut self) -> Result<Box<dyn MappedImageDataMut + '_>, ImageDataError> {
        Ok(Box::new(MappedGpuImageMut::map(self)?))
    }
}
