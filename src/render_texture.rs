use gl::types::GLuint;

use crate::Texture;

/// Represents a render texture, which allows rendering on. Can be used like a [`ferrousgl::texture::Texture`].
pub struct RenderTexture {
    framebuffer_id: GLuint,
    texture: Texture,
    width: u32,
    height: u32,
}

impl RenderTexture {
    /// Creates a new render texture with the specified width and height.
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let mut framebuffer_id = 0;
        let texture = Texture::new_empty(width, height)?;

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

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err("Framebuffer is not complete!".to_string());
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(RenderTexture {
            framebuffer_id,
            texture,
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

    /// Returns a reference to the texture that this render texture renders to.
    pub fn texture(&self) -> &Texture {
        &self.texture
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