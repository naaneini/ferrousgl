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
    pub fn new_from_file(path: &Path) -> Result<Self, String> {
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

    /// Sets the preferred Texture Mipmap Type or Texture Filtering Mode such as None, Linear and Nearest.
    /// Do this after binding a texture, otherwise it will not take effect.
    pub fn set_mipmap_and_filtering(&self, mipmap_type: MipmapType, base_filter: FilterMode) {
        unsafe {
            // Set minification filter based on both mipmap type and base filter
            match (mipmap_type, base_filter) {
                (MipmapType::None, FilterMode::Linear) => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                }
                (MipmapType::None, FilterMode::Nearest) => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                }
                (MipmapType::Nearest, FilterMode::Linear) => {
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::NEAREST_MIPMAP_LINEAR as i32,
                    );
                }
                (MipmapType::Nearest, FilterMode::Nearest) => {
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::NEAREST_MIPMAP_NEAREST as i32,
                    );
                }
                (MipmapType::Linear, FilterMode::Linear) => {
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::LINEAR_MIPMAP_LINEAR as i32,
                    );
                }
                (MipmapType::Linear, FilterMode::Nearest) => {
                    gl::TexParameteri(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::LINEAR_MIPMAP_NEAREST as i32,
                    );
                }
            }

            // Set magnification filter (mipmaps don't affect magnification)
            match base_filter {
                FilterMode::Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                }
                FilterMode::Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                }
            }
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

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
    // Bind the texture
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    // First try to read as RGBA texture
    let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
    
    unsafe {
        gl::GetTexImage(
            gl::TEXTURE_2D,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            buffer.as_mut_ptr() as *mut GLvoid,
        );
    }

    // Check if we got meaningful RGBA data (not all zeros)
    let is_depth_texture = buffer.iter().all(|&x| x == 0);
    
    if is_depth_texture {
        // If it's a depth texture, read it as depth data
        let mut depth_buffer = vec![0f32; (self.width * self.height) as usize];
        
        unsafe {
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                depth_buffer.as_mut_ptr() as *mut GLvoid,
            );
            
            // Unbind the texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        // Convert depth values to red-black gradient
        let mut image_buffer = Vec::with_capacity((self.width * self.height * 4) as usize);
        
        for depth in depth_buffer {
            // Normalize depth value (assuming it's in [0,1] range)
            let depth = depth.clamp(0.0, 1.0);
            
            // Convert to red-black gradient
            image_buffer.push((depth * 255.0) as u8); // R
            image_buffer.push(0);                     // G
            image_buffer.push(0);                     // B
            image_buffer.push(255);                   // A
        }

        // Create and save the image
        match image::RgbaImage::from_raw(self.width, self.height, image_buffer) {
            Some(image) => {
                image.save(path).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Err("Failed to create image from depth texture data".to_string()),
        }
    } else {
        // Regular RGBA texture
        unsafe {
            // Unbind the texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        match image::RgbaImage::from_raw(self.width, self.height, buffer) {
            Some(image) => {
                image.save(path).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Err("Failed to create image from texture data".to_string()),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MipmapType {
    None,
    Linear,
    Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Linear,
    Nearest,
}
