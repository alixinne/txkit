use std::rc::Rc;

use glutin::event_loop::EventLoop;
use glutin::{Context, ContextBuilder, PossiblyCurrent};

use tinygl::prelude::*;
use tinygl::wrappers::GlHandle;

use crate::image::{gpu::GpuImageData, ImageDim};
use crate::Result;

/// txkit internal context for GPU computations
#[allow(dead_code)]
pub struct GpuContext {
    /// gl function loader instance
    pub(crate) gl: Rc<tinygl::Context>,

    /// VAO for quad rendering
    pub(crate) vao: tinygl::wrappers::VertexArray,

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

    pub fn new() -> Result<Self> {
        let el = Self::get_event_loop();

        let sz = glutin::dpi::PhysicalSize::new(512, 512);

        let headless_context = ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl_debug_flag(true)
            .build_headless(&el, sz)?;

        let (gl, headless_context) = unsafe {
            let headless_context = headless_context.make_current().map_err(|(_ctx, err)| err)?;

            (
                Rc::new(tinygl::Context::from_loader_function(|s| {
                    headless_context.get_proc_address(s) as *const _
                })),
                headless_context,
            )
        };

        // Build an empty VAO for quad rendering
        let vao = tinygl::wrappers::VertexArray::new(&gl)?;

        let rtt = TextureRenderTarget::new(&gl)?;

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
        tgt: &mut GpuImageData,
        mut f: impl FnMut(&Rc<tinygl::Context>, u32) -> Result<()>,
    ) -> Result<()> {
        // Setup framebuffer
        let dim = tgt.dim;

        // Set target framebuffer
        self.rtt
            .framebuffer
            .bind(&*self.gl, tinygl::gl::FRAMEBUFFER);

        self.rtt.alloc(&self.gl, dim);

        unsafe {
            // Set viewport
            self.gl.viewport(0, 0, dim.width as i32, dim.height as i32);
        }

        // Bind VAO
        self.vao.bind(&self.gl);

        let mut r = Ok(());

        match tgt.target() {
            tinygl::gl::TEXTURE_2D => {
                tgt.texture.bind(&*self.gl, tinygl::gl::TEXTURE_2D);

                // Set texture
                self.rtt.framebuffer.texture_2d(
                    &*self.gl,
                    tinygl::gl::FRAMEBUFFER,
                    tinygl::gl::COLOR_ATTACHMENT0,
                    tinygl::gl::TEXTURE_2D,
                    Some(&tgt.texture),
                    0,
                );
            }
            tinygl::gl::TEXTURE_3D => {
                tgt.texture.bind(&*self.gl, tinygl::gl::TEXTURE_3D);
            }
            _ => panic!("invalid texture target"),
        }

        for layer in 0..dim.depth {
            match tgt.target() {
                tinygl::gl::TEXTURE_2D => {}
                tinygl::gl::TEXTURE_3D => {
                    // Set texture
                    self.rtt.framebuffer.texture_3d(
                        &*self.gl,
                        tinygl::gl::FRAMEBUFFER,
                        tinygl::gl::COLOR_ATTACHMENT0,
                        tinygl::gl::TEXTURE_3D,
                        Some(&tgt.texture),
                        0,
                        layer as i32,
                    );
                }
                _ => panic!("invalid texture target"),
            }

            // Setup draw buffers
            //self.gl.draw_buffers(&[tinygl::gl::COLOR_ATTACHMENT0]);

            // Call rendering method
            r = f(&self.gl, layer as u32);
            if r.is_err() {
                // Abort rendering on first layer error
                break;
            }
        }

        // We changed the contents of the texture on GPU, it needs
        // to be downloaded before being mapped again
        tgt.invalidate_host();

        // Cleanup
        // Unbind texture from framebuffer
        self.rtt.framebuffer.texture(
            &*self.gl,
            tinygl::gl::FRAMEBUFFER,
            tinygl::gl::COLOR_ATTACHMENT0,
            None,
            0,
        );

        unsafe {
            self.gl.bind_texture(tgt.target(), None);
            self.gl.bind_vertex_array(None);
            self.gl.bind_framebuffer(tinygl::gl::FRAMEBUFFER, None);
        }

        r
    }
}

pub struct TextureRenderTarget {
    pub framebuffer: GlHandle<tinygl::wrappers::Framebuffer>,
    pub depthbuffer: GlHandle<tinygl::wrappers::Renderbuffer>,
    current_size: Option<cgmath::Vector2<i32>>,
}

impl TextureRenderTarget {
    pub fn new(gl: &Rc<tinygl::Context>) -> Result<TextureRenderTarget> {
        // Create objects
        Ok(Self {
            framebuffer: GlHandle::new(gl, tinygl::wrappers::Framebuffer::new(gl)?),
            depthbuffer: GlHandle::new(gl, tinygl::wrappers::Renderbuffer::new(gl)?),
            current_size: None,
        })
    }

    pub fn alloc(&mut self, gl: &Rc<tinygl::Context>, dim: ImageDim) {
        let new_size = cgmath::vec2(dim.width as i32, dim.height as i32);

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
            }

            if self.current_size.is_none() {
                self.framebuffer.renderbuffer(
                    gl,
                    tinygl::gl::FRAMEBUFFER,
                    tinygl::gl::DEPTH_ATTACHMENT,
                    Some(&self.depthbuffer),
                );
            }

            // Update size
            self.current_size = Some(new_size);
        }
    }
}
