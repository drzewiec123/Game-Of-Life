use gl;
use std;
use std::ffi::{CString, CStr};
use crate::utils::*;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(kind) };
    
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
    
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
    
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Shader{id})
    }

    pub fn from_vert_cstr(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_vert_str(source: &str) -> Result<Shader, String> {
        Self::from_vert_cstr(&CString::new(source).unwrap())
    }

    pub fn from_frag_cstr(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_frag_str(source: &str) -> Result<Shader, String> {
        Self::from_frag_cstr(&CString::new(source).unwrap())
    }

    pub fn from_comp_cstr(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::COMPUTE_SHADER)
    }

    pub fn from_comp_str(source: &str) -> Result<Shader, String> {
        Self::from_comp_cstr(&CString::new(source).unwrap())
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
