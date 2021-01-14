use std::cell::RefCell;
use std::rc::Rc;

use tinygl::wrappers::{Buffer, GlRefHandle, Texture};

use super::*;
use crate::Error;

pub struct GpuImageData {
    gl: Rc<tinygl::Context>,
    pub(crate) texture: Texture,
    pub(crate) buffer: Buffer,
    pub(crate) element_type: ImageDataType,
    pub(crate) dim: ImageDim,

    target: u32,
    transfer_sync: RefCell<Option<tinygl::gl::Fence>>,
}

impl GpuImageData {
    fn new_nd(
        gl: &Rc<tinygl::Context>,
        dim: ImageDim,
        element_type: ImageDataType,
        target: u32,
        allocator: impl Fn(ImageDim, ImageDataType) -> Result<(), ImageCreationError>,
    ) -> Result<Self, ImageCreationError> {
        let texture = GlRefHandle::new(&*gl, Texture::new(gl)?);
        let buffer = GlRefHandle::new(&*gl, Buffer::new(gl)?);

        unsafe {
            texture.bind(gl, target);

            // Allocation result
            let res = {
                gl.tex_parameteri(
                    target,
                    tinygl::gl::TEXTURE_MIN_FILTER,
                    tinygl::gl::LINEAR as i32,
                );

                gl.tex_parameteri(
                    target,
                    tinygl::gl::TEXTURE_MAG_FILTER,
                    tinygl::gl::LINEAR as i32,
                );

                allocator(dim, element_type)?;

                // Check that the allocation succeeded
                gl.check_last_error()
            };

            // Unbind texture after allocation
            gl.bind_texture(target, None);

            // Bind buffer for initialization
            buffer.bind(&*gl, tinygl::gl::PIXEL_PACK_BUFFER);

            // Only needed once since image sizes are immutable
            gl.buffer_data(
                tinygl::gl::PIXEL_PACK_BUFFER,
                Self::calc_byte_size(element_type, dim) as isize,
                std::ptr::null(),
                tinygl::gl::DYNAMIC_READ,
            );

            gl.check_last_error()?;

            // Unbind buffer
            gl.bind_buffer(tinygl::gl::PIXEL_PACK_BUFFER, None);

            res?
        };

        Ok(Self {
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
    ) -> Result<Self, ImageCreationError> {
        if dim.height > 1 || dim.depth > 1 {
            return Err(ImageCreationError::InvalidImageSize);
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
                        dim.internal_format(element_type)
                            .ok_or_else(|| ImageCreationError::InvalidChannelCount(dim.channels))?,
                        dim.width as i32,
                        0,
                        dim.unsized_format()
                            .ok_or_else(|| ImageCreationError::InvalidChannelCount(dim.channels))?,
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
    ) -> Result<Self, ImageCreationError> {
        if dim.depth > 1 {
            return Err(ImageCreationError::InvalidImageSize);
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
                        dim.internal_format(element_type)
                            .ok_or_else(|| ImageCreationError::InvalidChannelCount(dim.channels))?,
                        dim.width as i32,
                        dim.height as i32,
                        0,
                        dim.unsized_format()
                            .ok_or_else(|| ImageCreationError::InvalidChannelCount(dim.channels))?,
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
    ) -> Result<Self, ImageCreationError> {
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
                        dim.internal_format(element_type)
                            .ok_or_else(|| ImageCreationError::InvalidChannelCount(dim.channels))?,
                        dim.width as i32,
                        dim.height as i32,
                        dim.depth as i32,
                        0,
                        dim.unsized_format()
                            .ok_or_else(|| ImageCreationError::InvalidChannelCount(dim.channels))?,
                        element_type.format_type(),
                        None,
                    );
                }

                Ok(())
            },
        )
    }

    pub fn target(&self) -> u32 {
        self.target
    }

    pub fn byte_size(&self) -> usize {
        Self::calc_byte_size(self.element_type, self.dim)
    }

    fn calc_byte_size(element_type: ImageDataType, dim: ImageDim) -> usize {
        element_type.byte_size() * dim.width * dim.height * dim.depth * dim.channels
    }

    fn start_download(&mut self) -> Result<(), Error> {
        unsafe {
            self.buffer.bind(&*self.gl, tinygl::gl::PIXEL_PACK_BUFFER);

            self.texture.bind(&*self.gl, self.target);
            self.gl.get_tex_image(
                self.target,
                0,
                self.dim.unsized_format().unwrap(),
                self.element_type.format_type(),
                std::ptr::null_mut(),
            );

            self.gl.bind_buffer(tinygl::gl::PIXEL_PACK_BUFFER, None);

            *self.transfer_sync.borrow_mut() = Some({
                let fence = self
                    .gl
                    .fence_sync(tinygl::gl::SYNC_GPU_COMMANDS_COMPLETE, 0);

                if fence == std::ptr::null() {
                    return Err(Error::OpenGlError(tinygl::Error::OpenGlError(
                        tinygl::OpenGlErrorCode(self.gl.get_error()),
                    )));
                }

                fence
            });

            self.gl.bind_texture(self.target, None);

            Ok(())
        }
    }

    fn start_upload(&mut self) -> Result<(), Error> {
        unsafe {
            self.buffer.bind(&*self.gl, tinygl::gl::PIXEL_UNPACK_BUFFER);
            self.texture.bind(&*self.gl, self.target);

            match self.target {
                tinygl::gl::TEXTURE_1D => {
                    self.gl.tex_image_1d(
                        self.target,
                        0,
                        self.dim
                            .internal_format(self.element_type)
                            .expect("incompatible internal format"),
                        self.dim.width as _,
                        0,
                        self.dim
                            .unsized_format()
                            .expect("incompatible unsized format"),
                        self.element_type.format_type(),
                        None,
                    );
                }
                tinygl::gl::TEXTURE_2D => {
                    self.gl.tex_image_2d(
                        self.target,
                        0,
                        self.dim
                            .internal_format(self.element_type)
                            .expect("incompatible internal format"),
                        self.dim.width as _,
                        self.dim.height as _,
                        0,
                        self.dim
                            .unsized_format()
                            .expect("incompatible unsized format"),
                        self.element_type.format_type(),
                        None,
                    );
                }
                tinygl::gl::TEXTURE_3D => {
                    self.gl.tex_image_3d(
                        self.target,
                        0,
                        self.dim
                            .internal_format(self.element_type)
                            .expect("incompatible internal format"),
                        self.dim.width as _,
                        self.dim.height as _,
                        self.dim.depth as _,
                        0,
                        self.dim
                            .unsized_format()
                            .expect("incompatible unsized format"),
                        self.element_type.format_type(),
                        None,
                    );
                }
                _ => unreachable!("unknown texture target"),
            }

            self.gl.check_last_error()?;

            self.gl.bind_texture(self.target, None);
            self.gl.bind_buffer(tinygl::gl::PIXEL_UNPACK_BUFFER, None);

            Ok(())
        }
    }

    unsafe fn map_buffer(&self, usage: u32) -> Result<*mut u8, ImageDataError> {
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
            self.byte_size() as isize,
            usage,
        );

        self.gl.bind_buffer(tinygl::gl::PIXEL_PACK_BUFFER, None);

        if ptr == std::ptr::null_mut() {
            Err(ImageDataError::MappingFailed)
        } else {
            Ok(ptr as *mut u8)
        }
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

        unsafe {
            self.texture.drop(&*self.gl);
            self.buffer.drop(&*self.gl);
        }
    }
}

impl ImageDataBase for GpuImageData {
    fn dim(&self) -> ImageDim {
        self.dim
    }
    fn element_type(&self) -> ImageDataType {
        self.element_type
    }
    fn download(&mut self) -> Result<(), Error> {
        self.start_download()
    }
    fn upload(&mut self) -> Result<(), Error> {
        self.start_upload()
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
    fn map(tgt: &'t GpuImageData) -> std::result::Result<Self, ImageDataError> {
        let mapped_ptr = unsafe { tgt.map_buffer(tinygl::gl::MAP_READ_BIT)? };

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
    fn map(tgt: &'t mut GpuImageData) -> std::result::Result<Self, ImageDataError> {
        let mapped_ptr =
            unsafe { tgt.map_buffer(tinygl::gl::MAP_READ_BIT | tinygl::gl::MAP_WRITE_BIT)? };

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
    fn data(&self) -> std::result::Result<Box<dyn MappedImageData + '_>, ImageDataError> {
        Ok(Box::new(MappedGpuImage::map(self)?))
    }

    fn data_mut(
        &mut self,
    ) -> std::result::Result<Box<dyn MappedImageDataMut + '_>, ImageDataError> {
        Ok(Box::new(MappedGpuImageMut::map(self)?))
    }
}
