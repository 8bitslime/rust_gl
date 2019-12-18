#![allow(dead_code)]

use gl::types::*;
use std::mem::size_of;

const MAX_BUFFERS: usize = 8;

pub struct Mesh {
    vao: GLuint,
    vbo: [GLuint; MAX_BUFFERS],
    num_buffers: usize
}
impl Mesh {
    pub fn new(buffers: usize) -> Mesh {
        unsafe {
            let mut mesh = Mesh {
                vao: 0,
                vbo: [0; MAX_BUFFERS],
                num_buffers: buffers
            };
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::BindVertexArray(mesh.vao);
            gl::GenBuffers(buffers as GLsizei, mesh.vbo.as_mut_ptr());
            mesh
        }
    }
    
    pub unsafe fn buffer_data_f(&self, buffer: usize, vec_size: usize, stride: usize, length: usize, data: *const f32) {
        debug_assert!(buffer < MAX_BUFFERS);
        let f32size: usize = size_of::<f32>();
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo[buffer]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (length * stride * f32size) as GLsizeiptr,
            data as *const _,
            gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(buffer as GLuint);
        gl::VertexAttribPointer(
            buffer as GLuint,
            vec_size as GLint,
            gl::FLOAT,
            gl::FALSE,
            (stride * f32size) as GLsizei,
            std::ptr::null());
    }
    pub fn buffer_data_1f(&self, buffer: usize, data: &[f32]) {
        unsafe { self.buffer_data_f(buffer, 1, 1, data.len(), data.as_ptr()) };
    }
    pub fn buffer_data_2f(&self, buffer: usize, data: &[glam::Vec2]) {
        const SIZE: usize = size_of::<glam::Vec2>() / size_of::<f32>();
        unsafe { self.buffer_data_f(buffer, 2, SIZE, data.len(), data.as_ptr() as *const f32) };
    }
    pub fn buffer_data_3f(&self, buffer: usize, data: &[glam::Vec3]) {
        //Check to see if vec3 is a __m128 or [f32; 3]
        const SIZE: usize = size_of::<glam::Vec3>() / size_of::<f32>();
        unsafe { self.buffer_data_f(buffer, 3, SIZE, data.len(), data.as_ptr() as *const f32) };
    }
    pub fn buffer_data_4f(&self, buffer: usize, data: &[glam::Vec4]) {
        const SIZE: usize = size_of::<glam::Vec4>() / size_of::<f32>();
        unsafe { self.buffer_data_f(buffer, 4, SIZE, data.len(), data.as_ptr() as *const f32) };
    }
    
    pub fn element_buffer_data(&self, buffer: usize, data: &[u32]) {
        const USIZE: usize = size_of::<u32>();
        debug_assert!(buffer < MAX_BUFFERS);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo[buffer]);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * USIZE) as GLsizeiptr,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW);
        }
    }
    
    pub fn draw(&self, length: usize, offset: usize) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, offset as GLint, length as GLsizei);
        }
    }
    pub fn draw_elements(&self, length: usize, offset: usize) {
        const USIZE: usize = size_of::<u32>();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                length as GLsizei,
                gl::UNSIGNED_INT,
                (offset * USIZE) as *const _);
        }
    }
}
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.num_buffers as GLsizei, self.vbo.as_mut_ptr());
            gl::DeleteVertexArrays(1, &self.vao);
        };
    }
}
