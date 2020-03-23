use std::rc::Rc;

use glutin::event_loop::EventLoop;
use glutin::{Context, ContextBuilder, PossiblyCurrent};

use tinygl::prelude::*;
use tinygl::wrappers::GlHandle;

use crate::image::Image;

/// txkit internal context for GPU computations
#[allow(dead_code)]
pub struct GpuContext {
    /// gl function loader instance
    pub(crate) gl: Rc<tinygl::Context>,

    /// VAO for quad rendering
    pub(crate) vao: <tinygl::glow::Context as HasContext>::VertexArray,

    el: EventLoop<()>,
    context: Context<PossiblyCurrent>,

    /// Default render target
    rtt: TextureRenderTarget,
}

impl GpuContext {
    #[cfg(target_os = "linux")]
    fn get_event_loop() -> EventLoop<()> {
        glutin::platform::unix::EventLoopExtUnix::new_any_thread()
    }

    #[cfg(not(target_os = "linux"))]
    fn get_event_loop() -> EventLoop<()> {
        EventLoop::new()
    }

    pub fn new() -> Result<Self, String> {
        let el = Self::get_event_loop();

        let sz = glutin::dpi::PhysicalSize::new(512, 512);

        let headless_context = ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl_debug_flag(true)
            .build_headless(&el, sz)
            .map_err(|_e| "failed to initialize OpenGL context".to_owned())?;

        let (gl, headless_context) = unsafe {
            let headless_context = headless_context
                .make_current()
                .map_err(|_e| "failed to make OpenGL context current".to_owned())?;

            (
                Rc::new(tinygl::Context::from_loader_function(|s| {
                    headless_context.get_proc_address(s) as *const _
                })),
                headless_context,
            )
        };

        // Build an empty VAO for quad rendering
        let vao = unsafe { gl.create_vertex_array() }?;

        let rtt = TextureRenderTarget::new(&gl, 256, 256)?;

        Ok(Self {
            el,
            context: headless_context,
            gl,
            vao,
            rtt,
        })
    }

    pub fn render_to_framebuffer(
        &mut self,
        tgt: &mut Image,
        f: impl FnOnce(&Rc<tinygl::Context>) -> Result<(), crate::method::Error>,
    ) -> Result<(), crate::method::Error> {
        // Setup framebuffer
        // TODO: Proper channels and element type
        self.rtt.alloc(&self.gl, tgt.width(), tgt.height());

        // Set target framebuffer
        self.rtt
            .framebuffer
            .bind(&*self.gl, tinygl::gl::FRAMEBUFFER);

        unsafe {
            // Set viewport
            self.gl
                .viewport(0, 0, tgt.width() as i32, tgt.height() as i32);

            // Bind VAO
            self.gl.bind_vertex_array(Some(self.vao));
        }

        // Call rendering method
        let r = match f(&self.gl) {
            Ok(_) => {
                // Fetch result into image
                self.rtt
                    .texture_main
                    .bind(&*self.gl, tinygl::gl::TEXTURE_2D);

                unsafe {
                    // TODO: Check slice construction for ndarray
                    self.gl.get_tex_image_u8_slice(
                        tinygl::gl::TEXTURE_2D,
                        0,
                        tinygl::gl::RGBA,
                        tinygl::gl::FLOAT,
                        Some(std::slice::from_raw_parts(
                            tgt.as_ptr() as *const u8,
                            tgt.width()
                                * tgt.height()
                                * tgt.depth()
                                * tgt.channels()
                                * std::mem::size_of::<f32>(),
                        )),
                    );

                    // Unbind texture
                    self.gl.bind_texture(tinygl::gl::TEXTURE_2D, None);
                }

                Ok(())
            }
            other => other,
        };

        // Cleanup
        unsafe {
            self.gl.bind_vertex_array(None);
            self.gl.bind_framebuffer(tinygl::gl::FRAMEBUFFER, None);
        }

        r
    }
}

pub struct TextureRenderTarget {
    pub framebuffer: GlHandle<tinygl::wrappers::Framebuffer>,
    pub depthbuffer: GlHandle<tinygl::wrappers::Renderbuffer>,
    pub texture_main: GlHandle<tinygl::wrappers::Texture>,
    current_size: Option<cgmath::Vector2<i32>>,
}

impl TextureRenderTarget {
    pub fn new(
        gl: &Rc<tinygl::Context>,
        width: usize,
        height: usize,
    ) -> Result<TextureRenderTarget, String> {
        // Create objects
        let mut this = Self {
            framebuffer: GlHandle::new(gl, tinygl::wrappers::Framebuffer::new(gl)?),
            depthbuffer: GlHandle::new(gl, tinygl::wrappers::Renderbuffer::new(gl)?),
            texture_main: GlHandle::new(gl, tinygl::wrappers::Texture::new(gl)?),
            current_size: None,
        };

        // Initial allocation
        this.alloc(gl, width, height);

        // Don't use mipmaps
        unsafe {
            for tex in [&this.texture_main].iter() {
                tex.bind(gl, tinygl::gl::TEXTURE_2D);
                gl.tex_parameter_i32(
                    tinygl::gl::TEXTURE_2D,
                    tinygl::gl::TEXTURE_MIN_FILTER,
                    tinygl::gl::NEAREST as i32,
                );
                gl.tex_parameter_i32(
                    tinygl::gl::TEXTURE_2D,
                    tinygl::gl::TEXTURE_MAG_FILTER,
                    tinygl::gl::NEAREST as i32,
                );
            }

            gl.bind_texture(tinygl::gl::TEXTURE_2D, None);
        }

        // Setup bindings
        unsafe {
            this.framebuffer.bind(gl, tinygl::gl::FRAMEBUFFER);
            this.framebuffer.renderbuffer(
                gl,
                tinygl::gl::FRAMEBUFFER,
                tinygl::gl::DEPTH_ATTACHMENT,
                Some(&this.depthbuffer),
            );
            this.framebuffer.texture(
                gl,
                tinygl::gl::FRAMEBUFFER,
                tinygl::gl::COLOR_ATTACHMENT0,
                Some(&this.texture_main),
                0,
            );
            gl.draw_buffers(&[tinygl::gl::COLOR_ATTACHMENT0, tinygl::gl::COLOR_ATTACHMENT1]);
            gl.bind_framebuffer(tinygl::gl::FRAMEBUFFER, None);
        }

        Ok(this)
    }

    pub fn alloc(&mut self, gl: &Rc<tinygl::Context>, width: usize, height: usize) {
        let new_size = cgmath::vec2(width as i32, height as i32);

        if !self.current_size.map(|cs| cs == new_size).unwrap_or(false) {
            // Setup storage
            unsafe {
                // Depth buffer
                self.depthbuffer.bind(gl);
                gl.renderbuffer_storage(
                    tinygl::gl::RENDERBUFFER,
                    tinygl::gl::DEPTH_COMPONENT,
                    new_size.x,
                    new_size.y,
                );
                gl.bind_renderbuffer(tinygl::gl::RENDERBUFFER, None);

                // Textures
                for tex in [&self.texture_main].iter() {
                    tex.bind(gl, tinygl::gl::TEXTURE_2D);
                    gl.tex_image_2d(
                        tinygl::gl::TEXTURE_2D,
                        0,
                        tinygl::gl::RGBA32F as i32,
                        new_size.x,
                        new_size.y,
                        0,
                        tinygl::gl::RGBA,
                        tinygl::gl::FLOAT,
                        None,
                    );
                }

                gl.bind_texture(tinygl::gl::TEXTURE_2D, None);
            }

            // Update size
            self.current_size = Some(new_size);
        }
    }
}
