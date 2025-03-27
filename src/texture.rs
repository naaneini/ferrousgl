extern crate gl;
extern crate image;

use gl::types::*;
use image::DynamicImage;
use std::path::Path;
use std::ptr;

/// Represents an OpenGL texture.
pub struct Texture {
    pub(crate) id: GLuint,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Texture {
    /// Creates a new texture from an image file.
    pub fn new_from_file(path: &str) -> Result<Self, String> {
        let img = image::open(&Path::new(path)).map_err(|e| e.to_string())?;
        Self::from_image(&img)
    }

    /// Creates a new texture from an in-memory image.
    pub fn from_image(img: &DynamicImage) -> Result<Self, String> {
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();

        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const GLvoid,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture {
            id: texture_id,
            width,
            height,
        })
    }

    /// Creates an empty texture with the specified width and height.
    pub fn new_empty(width: u32, height: u32) -> Result<Self, String> {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture {
            id: texture_id,
            width,
            height,
        })
    }

    /// Binds the texture to a specific texture unit which can be used to set a uniform texture.
    pub fn bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbinds the texture.
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// Allows the user to enable the mipmap mode.
    pub fn enable_mipmap_mode(&self, enable_mipmaps: bool) {
        unsafe {
            Texture::bind(&self, 0);
            if enable_mipmaps {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            }
            Texture::unbind(&self);
        }
    }

    /// Returns the width of the texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the texture.
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture {
    /// Cleans up the texture when it goes out of scope.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
