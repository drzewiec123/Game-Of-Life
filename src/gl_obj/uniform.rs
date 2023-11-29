use std::marker::PhantomData;
use std::ffi::CStr;
use std::slice;

use glam;
use gl;

pub trait UniformType {
    fn send_to(&self, location: gl::types::GLint);
}

pub struct Uniform<'a, UnifType: UniformType> {

    location: gl::types::GLint,
    _lifetime: PhantomData<&'a ()>,
    _unif_type: PhantomData<UnifType>

}

impl<UnifType: UniformType> Uniform<'_, UnifType> {

    pub fn from_name<'a>(program_id: gl::types::GLuint, name: &CStr) -> Result<Uniform<'a, UnifType>, String> {
        let loc = unsafe { gl::GetUniformLocation(program_id, name.as_ptr()) };
        if loc == -1 {
            Err("Could not find uniform: ".to_string() + &name.to_string_lossy())
        } else {
            Ok(Uniform {
                location: loc,
                _lifetime: PhantomData{},
                _unif_type: PhantomData{}
            })
        }
    }

    pub fn set(&self, value: &UnifType) {
        value.send_to(self.location);
    }
   
}

impl UniformType for glam::Vec3 {
    fn send_to(&self, location: gl::types::GLint) {
        unsafe { gl::Uniform3fv(location, 1, slice::from_ref(self).as_ptr() as *const gl::types::GLfloat); }
    }
}

impl UniformType for glam::Mat2 {
    fn send_to(&self, location: gl::types::GLint) {
        unsafe { gl::UniformMatrix2fv(location, 1, gl::FALSE, slice::from_ref(self).as_ptr() as *const gl::types::GLfloat); }
    }
}

impl UniformType for glam::Mat4 {
    fn send_to(&self, location: gl::types::GLint) {
        unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, slice::from_ref(self).as_ptr() as *const gl::types::GLfloat); }
    }
}

impl UniformType for gl::types::GLuint {
    fn send_to(&self, location: gl::types::GLint) {
        unsafe { gl::Uniform1ui(location, *self); }
    }
}

impl UniformType for gl::types::GLint {
    fn send_to(&self, location: gl::types::GLint) {
        unsafe { gl::Uniform1i(location, *self); }
    }
}
