
use gl::types::*;
use glam::{Vec2, Vec3, Vec4, Mat4};
use std::ffi::CStr;
use std::ffi::CString;

pub struct Shader(GLuint);
impl Shader {
    pub fn create_header(header: &str, source: &str, shader_type: GLenum) -> Result<Shader, String> {
        let strings = [
            header.as_ptr() as *const GLchar,
            source.as_ptr() as *const GLchar,
        ];
        
        let lengths = [
            header.len() as GLint,
            source.len() as GLint,
        ];
        
        unsafe {
            let shader = gl::CreateShader(shader_type);
            
            gl::ShaderSource(shader, 2, strings.as_ptr(), lengths.as_ptr());
            gl::CompileShader(shader);
            
            let mut success = i32::from(gl::FALSE);
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                let mut info_log = [0; 512];
                gl::GetShaderInfoLog(
                    shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar);
                
                let string = CStr::from_ptr(info_log.as_ptr());
                Err(string.to_str().unwrap().to_owned())
            } else {
                Ok(Shader(shader))
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn create(source: &str, shader_type: GLenum) -> Result<Shader, String> {
        Shader::create_header("", source, shader_type)
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}

pub trait SetUniform<T> {
    fn set_uniform(&self, location: Uniform, value: &T);
}

pub struct Program(GLuint);
type Uniform = GLint;
impl Program {
    pub fn new() -> Program {
        Program(unsafe { gl::CreateProgram() })
    }
    pub fn from_source(source: &str) -> Result<Program, String> {
        let vertex = Shader::create_header(
            "#version 450 core\n#define VERTEX\n",
            source,
            gl::VERTEX_SHADER)?;
        
        let fragment = Shader::create_header(
            "#version 450 core\n#define FRAGMENT\n",
            source,
            gl::FRAGMENT_SHADER)?;
        
        let program = Program::new();
        program.attach(&vertex);
        program.attach(&fragment);
        let program = program.link()?;
        program.detach(&vertex);
        program.detach(&fragment);
        Ok(program)
    }
    pub fn attach(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.0, shader.0);
        };
    }
    pub fn detach(&self, shader: &Shader) {
        unsafe {
            gl::DetachShader(self.0, shader.0);
        };
    }
    pub fn link(self) -> Result<Program, String> {
        unsafe {
            gl::LinkProgram(self.0);
            let mut info_log = [0; 512];
            let mut success = i32::from(gl::FALSE);
            gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetProgramInfoLog(
                    self.0,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar);
                
                let string = CStr::from_bytes_with_nul_unchecked(&info_log);
                Err(string.to_str().unwrap().to_owned())
            } else {
                Ok(self)
            }
        }
    }
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.0) };
    }
    
    pub fn get_uniform(&self, name: &str) -> Option<Uniform> {
        unsafe {
            let string = CString::new(name).unwrap();
            let uniform = gl::GetUniformLocation(self.0, string.as_ptr());
            
            if uniform >= 0 {
                Some(uniform)
            } else {
                None
            }
        }
    }
}

impl SetUniform<Vec2> for Program {
    fn set_uniform(&self, location: Uniform, value: &Vec2) {
        unsafe { gl::Uniform2f(location, value.x(), value.y()) };
    }
}
impl SetUniform<Vec3> for Program {
    fn set_uniform(&self, location: Uniform, value: &Vec3) {
        unsafe { gl::Uniform3f(location, value.x(), value.y(), value.z()) };
    }
}
impl SetUniform<Vec4> for Program {
    fn set_uniform(&self, location: Uniform, value: &Vec4) {
        unsafe { gl::Uniform4f(location, value.x(), value.y(), value.z(), value.w()) };
    }
}
impl SetUniform<Mat4> for Program {
    fn set_uniform(&self, location: Uniform, value: &Mat4) {
        unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ref().as_ptr()) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) };
    }
}
