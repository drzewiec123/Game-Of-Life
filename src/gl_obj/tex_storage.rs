use gl;

pub struct TexStorage {
    id: gl::types::GLuint,
    format: u32,
    width: i32,
    height: i32
}

impl TexStorage {

    pub fn new(width: i32, height: i32, format: u32) -> TexStorage {
        let mut tex_storage = TexStorage{id: 0, format, width, height};
        unsafe {
            gl::GenTextures(1, &mut tex_storage.id);
            tex_storage.bind(gl::TEXTURE1);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, format, width, height);
        }
        tex_storage
    }

    pub fn bind(&self, target: u32) {
        unsafe {
            gl::ActiveTexture(target);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn bind_image_texture(&self, unit: u32, access: u32) {
        unsafe {
            gl::BindImageTexture(unit, self.id, 0, 0, 0, access, self.format);
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn fill<T>(&self, data: &[T], format: gl::types::GLenum, type_: gl::types::GLenum) {
        self.bind(gl::TEXTURE0);
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, self.width, self.height, 
                format, type_, data.as_ptr() as *const gl::types::GLvoid);
        }
    }

}

impl Drop for TexStorage {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
