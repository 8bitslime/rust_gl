use gl;
use gl::types::*;
use std::path::Path;
use image::GenericImageView;
use image::ImageError;

pub struct Texture(GLuint);

impl Texture {
    pub fn from_path(name: &Path) -> Result<Self, ImageError> {
        let img = image::open(name)?;
        
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                img.raw_pixels().as_ptr() as *const _
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        };
        
        Ok(Texture(texture))
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.0) };
    }
}
