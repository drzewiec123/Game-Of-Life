use crate::{gl_obj, registry::program_registry::ProgramRegistry};

pub struct BoardState<'a> {
    primary_board: gl_obj::TexStorage,
    secondary_board: gl_obj::TexStorage,
    board_step_prog: &'a gl_obj::Program,
}

impl<'a> BoardState<'a> {

    pub fn new<'b>(program_registry: &'b ProgramRegistry, width: i32, height: i32, data: &[gl::types::GLubyte]) -> Result<BoardState<'b>, String> {
        if width % 8 != 0 || height % 8 != 0 {
            return Err("board dimensions have to be multiples of 8".to_owned());
        }
        let primary = gl_obj::TexStorage::new(width, height, gl::R8UI);
        primary.fill(data, gl::RED_INTEGER, gl::UNSIGNED_BYTE);
        Ok(BoardState {
            primary_board: primary,
            secondary_board: gl_obj::TexStorage::new(width, height, gl::R8UI),
            board_step_prog: &program_registry.board_step_prog,
        })
    }

    pub fn get_current_board(&self) -> &gl_obj::TexStorage {
        &self.primary_board
    }

    pub fn step(&mut self) {
        self.board_step_prog.bind();
        self.primary_board.bind_image_texture(1, gl::READ_ONLY);
        self.secondary_board.bind_image_texture(2, gl::WRITE_ONLY);
        unsafe {
            gl::DispatchCompute(self.primary_board.width() as u32 / 8, self.primary_board.height() as u32 / 8, 1);
        }
        std::mem::swap(&mut self.primary_board, &mut self.secondary_board);
    }

}
