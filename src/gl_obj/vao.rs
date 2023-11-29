use gl;

pub struct Vao {
    id: gl::types::GLuint
}

impl Vao {

    pub fn new() -> Vao {
        let mut vao = Vao{id: 0};
        unsafe {
            gl::GenVertexArrays(1, &mut vao.id);
        }
        vao
    }

    pub fn attrib_pointer(&self,
        index: gl::types::GLuint,
        size: usize,
        type_: gl::types::GLenum,
        normalized: bool,
        stride: usize,
        offset: usize
    ) {
        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                size as gl::types::GLint,
                type_,
                if normalized { gl::TRUE } else { gl::FALSE },
                stride as gl::types::GLsizei,
                offset as *const std::ffi::c_void
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}