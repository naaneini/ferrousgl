use gl::types::GLuint;

use crate::Texture;

/// Represents a render texture, which allows rendering on. Can be used like a [`ferrousgl::texture::Texture`].
pub struct RenderTexture {
    framebuffer_id: GLuint,
    texture: Texture,
    depth_texture: Option<Texture>, // Added depth texture
    width: u32,
    height: u32,
}

impl RenderTexture {
    /// Creates a new render texture with the specified width and height.
    /// Optionally creates a depth texture attachment if `with_depth` is true.
    pub fn new(width: u32, height: u32, with_depth: bool) -> Result<Self, String> {
        let mut framebuffer_id = 0;
        let texture = Texture::new_empty(width, height)?;

        let depth_texture = if with_depth {
            Some(Self::create_depth_texture(width, height)?)
        } else {
            None
        };

        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.id,
                0,
            );

            if let Some(ref depth) = depth_texture {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    gl::TEXTURE_2D,
                    depth.id,
                    0,
                );
            }

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err("Framebuffer is not complete!".to_string());
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(RenderTexture {
            framebuffer_id,
            texture,
            depth_texture,
            width,
            height,
        })
    }

    /// Creates a depth texture with the specified dimensions
    fn create_depth_texture(width: u32, height: u32) -> Result<Texture, String> {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT as i32,
                width as i32,
                height as i32,
                0,
                gl::DEPTH_COMPONENT,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );

            let border_color = [1.0, 1.0, 1.0, 1.0];
            gl::TexParameterfv(
                gl::TEXTURE_2D,
                gl::TEXTURE_BORDER_COLOR,
                border_color.as_ptr(),
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture {
            id: texture_id,
            width,
            height,
        })
    }

    /// Binds the render texture as the current framebuffer.
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    /// Unbinds the render texture (binds the default framebuffer), after this normal rendering can continue.
    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// Returns a reference to the color texture that this render texture renders to.
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Returns a reference to the depth texture if it exists.
    pub fn depth_texture(&self) -> Option<&Texture> {
        self.depth_texture.as_ref()
    }
}

impl Drop for RenderTexture {
    /// Cleans up the framebuffer when it goes out of scope.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.framebuffer_id);
        }
    }
}
