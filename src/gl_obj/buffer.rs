use gl;

pub struct Buffer {
    id: gl::types::GLuint,
    target: gl::types::GLenum
}

impl Buffer {

    pub fn new(target: gl::types::GLenum) -> Buffer {
        let mut buffer = Buffer{id: 0, target};
        unsafe {
            gl::GenBuffers(1, &mut buffer.id);
        }
        buffer
    }

    pub fn buffer_data<T>(&self, data: &[T], usage: gl::types::GLenum) {
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                usage
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}