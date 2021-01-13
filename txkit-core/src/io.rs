use std::cell::RefCell;
use std::rc::Rc;

use crate::image::Image;

#[derive(Debug, Clone)]
pub enum ImageBinding {
    /// Empty binding
    None,
    /// Reference to an image
    ImageRef(Rc<RefCell<Image>>),
    /// Reference to an image for FFI
    ImagePtr(*mut Image),
}

impl Default for ImageBinding {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq for ImageBinding {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::None => matches!(other, Self::None),
            Self::ImageRef(self_rc) => match other {
                Self::ImageRef(other_rc) => self_rc.as_ptr() == other_rc.as_ptr(),
                _ => false,
            },
            Self::ImagePtr(self_ptr) => match other {
                Self::ImagePtr(other_ptr) => self_ptr == other_ptr,
                _ => false,
            },
        }
    }
}

// TODO: Detect at runtime?

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImageIo {
    /// Texture unit bindings
    texture_bindings: [ImageBinding; 32],
    /// Image unit bindings
    image_bindings: [ImageBinding; 32],
}

impl ImageIo {
    /// Create a new empty ImageIo object
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a texture binding from the given value
    ///
    /// # Parameters
    ///
    /// * `index`: unit index for the binding
    ///
    /// # Panics
    ///
    /// Panics if the given unit index is out of bounds.
    pub fn get_texture_binding(&self, index: usize) -> &ImageBinding {
        if index > self.texture_bindings.len() {
            panic!(
                "texture binding is out of range: {} > {}",
                index,
                self.texture_bindings.len()
            );
        }

        &self.texture_bindings[index]
    }

    /// Set a texture binding from the given value
    ///
    /// # Parameters
    ///
    /// * `index`: unit index for the binding
    /// * `binding`: binding object describing which image to bind
    ///
    /// # Panics
    ///
    /// Panics if the given unit index is out of bounds.
    pub fn set_texture_binding(&mut self, index: usize, binding: ImageBinding) {
        if index > self.texture_bindings.len() {
            panic!(
                "texture binding is out of range: {} > {}",
                index,
                self.texture_bindings.len()
            );
        }

        self.texture_bindings[index] = binding;
    }

    /// Get an image binding from the given value
    ///
    /// # Parameters
    ///
    /// * `index`: unit index for the binding
    ///
    /// # Panics
    ///
    /// Panics if the given unit index is out of bounds.
    pub fn get_image_binding(&self, index: usize) -> &ImageBinding {
        if index > self.image_bindings.len() {
            panic!(
                "image binding is out of range: {} > {}",
                index,
                self.image_bindings.len()
            );
        }

        &self.image_bindings[index]
    }

    /// Set an image binding from the given value
    ///
    /// # Parameters
    ///
    /// * `index`: unit index for the binding
    /// * `binding`: binding object describing which image to bind
    ///
    /// # Panics
    ///
    /// Panics if the given unit index is out of bounds.
    pub fn set_image_binding(&mut self, index: usize, binding: ImageBinding) {
        if index > self.image_bindings.len() {
            panic!(
                "image binding is out of range: {} > {}",
                index,
                self.image_bindings.len()
            );
        }

        self.image_bindings[index] = binding;
    }
}

#[cfg(feature = "gpu-core")]
pub mod gpu {
    use super::*;

    pub trait GpuImageIoExt {
        fn apply_image_binding(
            &self,
            gl: &tinygl::Context,
            index: usize,
            access: tinygl::gl::types::GLenum,
            format: tinygl::gl::types::GLenum,
        );
        fn apply_texture_binding(&self, gl: &tinygl::Context, index: usize);
    }

    impl GpuImageIoExt for ImageIo {
        fn apply_image_binding(
            &self,
            gl: &tinygl::Context,
            index: usize,
            access: tinygl::gl::types::GLenum,
            format: tinygl::gl::types::GLenum,
        ) {
            let binding = self.get_image_binding(index);

            unsafe {
                match binding {
                    ImageBinding::None => {
                        gl.bind_image_texture(index as _, None, 0, false, 0, access, format)
                    }
                    ImageBinding::ImageRef(img) => {
                        let img = img.borrow();
                        let gpu = img
                            .as_gpu_image()
                            .expect("a GPU image is required for an image binding");

                        gl.bind_image_texture(
                            index as _,
                            Some(&gpu.texture),
                            0,
                            false,
                            0,
                            access,
                            format,
                        );
                    }
                    ImageBinding::ImagePtr(img) => {
                        let gpu = img
                            .as_ref()
                            .expect("null pointer in GPU image ref")
                            .as_gpu_image()
                            .expect("a GPU image is required for an image binding");

                        gl.bind_image_texture(
                            index as _,
                            Some(&gpu.texture),
                            0,
                            false,
                            0,
                            access,
                            format,
                        );
                    }
                }
            }
        }

        fn apply_texture_binding(&self, gl: &tinygl::Context, index: usize) {
            let binding = self.get_image_binding(index);

            unsafe {
                match binding {
                    ImageBinding::None => {
                        gl.bind_texture_unit(index as _, 0);
                    }
                    ImageBinding::ImageRef(img) => {
                        let img = img.borrow();
                        let gpu = img
                            .as_gpu_image()
                            .expect("a GPU image is required for an image binding");

                        gl.bind_texture_unit(index as _, gpu.texture.name());
                    }
                    ImageBinding::ImagePtr(img) => {
                        let gpu = img
                            .as_ref()
                            .expect("null pointer in GPU image ref")
                            .as_gpu_image()
                            .expect("a GPU image is required for an image binding");

                        gl.bind_texture_unit(index as _, gpu.texture.name());
                    }
                }
            }
        }
    }
}
