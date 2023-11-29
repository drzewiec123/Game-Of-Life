use c_str_macro::c_str;
use glam::Vec2;

use crate::gl_obj;
use crate::registry::program_registry::ProgramRegistry;

pub struct BoardDisplay<'a> {
    transform: glam::Mat2,
    draw_texture_prog: &'a gl_obj::Program,
    _vertices: gl_obj::Buffer,
    vao: gl_obj::Vao,
    transform_unif: gl_obj::Uniform<'a, glam::Mat2>
}

impl<'a> BoardDisplay<'a> {

    pub fn new(program_registry: &ProgramRegistry, aspect_ratio: f32) -> Result<BoardDisplay, String>  {
        let vbo = gl_obj::Buffer::new(gl::ARRAY_BUFFER);
        let vao = gl_obj::Vao::new();
        vao.bind();
        vbo.bind();
        vao.attrib_pointer(0, 2, gl::FLOAT, false, std::mem::size_of::<Vec2>(), 0);
        vbo.buffer_data(&[
            Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0)
        ], gl::STATIC_DRAW);
        let board = BoardDisplay{
            transform: Self::get_transofrm_matrix(aspect_ratio),
            draw_texture_prog: &program_registry.draw_texture_prog,
            _vertices: vbo,
            vao,
            transform_unif: program_registry.draw_texture_prog.get_uniform(c_str!("uTransform"))?
        };
        Ok(board)
    }

    fn get_transofrm_matrix(aspect_ratio: f32) -> glam::Mat2 {
        if aspect_ratio < 1.0 {
            glam::Mat2::from_cols_array(&[1.0, 0.0, 0.0, 1.0 * aspect_ratio])
        } else {
            glam::Mat2::from_cols_array(&[1.0 / aspect_ratio, 0.0, 0.0, 1.0])
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.transform = Self::get_transofrm_matrix(aspect_ratio);
    }

    pub fn draw(&self, board_data: &gl_obj::TexStorage) {
        self.vao.bind();
        self.draw_texture_prog.bind();
        board_data.bind(gl::TEXTURE1);
        self.transform_unif.set(&self.transform);
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

}