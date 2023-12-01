use c_str_macro::c_str;
use sdl2::event::Event;

use crate::gl_obj;
use crate::registry::program_registry::ProgramRegistry;

pub struct ViewingState {
    center: glam::Vec2,
    zoom: f32,
}

impl ViewingState {
    const ZOOM_FACTOR: f32 = 0.9;

    pub fn new() -> ViewingState {
        ViewingState { center: glam::vec2(0.0, 0.0), zoom: 0.0 }
    }

    fn clamp_center_position(&mut self) {
        let allowed_offset = 1.0 - self.get_scaling_factor();
        self.center.x = self.center.x.clamp(-allowed_offset, allowed_offset);
        self.center.y = self.center.y.clamp(-allowed_offset, allowed_offset);
    }

    pub fn zoom(&mut self, value: f32) {
        self.zoom = f32::max(0.0, self.zoom + value);
        self.clamp_center_position()
    }

    pub fn mov(&mut self, value: glam::Vec2) {
        self.center += value;
        self.clamp_center_position()
    }

    pub fn get_texture_transform(&self) -> glam::Mat3 {
        let scale = self.get_scaling_factor();
        glam::Mat3::from_translation(self.center) * glam::Mat3::from_scale(glam::vec2(scale, scale))
    }

    pub fn get_scaling_factor(&self) -> f32 {
        Self::ZOOM_FACTOR.powf(self.zoom)
    }
}

struct Subwindow {
    main_window_size: glam::IVec2,
    position: glam::Vec2,
    dimensions: glam::Vec2
}

impl Subwindow {
    pub fn new(main_window_size: glam::IVec2, position: glam::Vec2, dimensions: glam::Vec2) -> Subwindow {
        Subwindow { main_window_size, position, dimensions }
    }

    pub fn set_main_window_size(&mut self, new_window_size: glam::IVec2) {
        self.main_window_size = new_window_size;
    }

    pub fn get_main_window_size(&self) -> glam::IVec2 {
        self.main_window_size
    }

    pub fn set_dimensions(&mut self, dimensions: glam::Vec2) {
        self.dimensions = dimensions
    }

    pub fn get_dimensions(&self) -> glam::Vec2 {
        self.dimensions
    }

    pub fn get_transofrm_matrix(&self) -> glam::Mat3 {
        glam::Mat3::from_scale(self.dimensions)
    }
}

pub struct BoardDisplay<'a> {
    draw_texture_prog: &'a gl_obj::Program,
    _vertices: gl_obj::Buffer,
    vao: gl_obj::Vao,
    transform_unif: gl_obj::Uniform<'a, glam::Mat3>,
    texture_transform_unif: gl_obj::Uniform<'a, glam::Mat3>,
    view_state: ViewingState,
    subwindow: Subwindow,
}

impl<'a> BoardDisplay<'a> {

    pub fn new(program_registry: &ProgramRegistry, window_dimensions: glam::IVec2) -> Result<BoardDisplay, String>  {
        let vbo = gl_obj::Buffer::new(gl::ARRAY_BUFFER);
        let vao = gl_obj::Vao::new();
        vao.bind();
        vbo.bind();
        vao.attrib_pointer(0, 2, gl::FLOAT, false, std::mem::size_of::<glam::Vec2>(), 0);
        vbo.buffer_data(&[
            glam::vec2(-1.0, -1.0), glam::vec2(1.0, -1.0), glam::vec2(1.0, 1.0), glam::vec2(-1.0, 1.0)
        ], gl::STATIC_DRAW);
        let mut board = BoardDisplay{
            draw_texture_prog: &program_registry.draw_texture_prog,
            _vertices: vbo,
            vao,
            transform_unif: program_registry.draw_texture_prog.get_uniform(c_str!("uTransform"))?,
            texture_transform_unif: program_registry.draw_texture_prog.get_uniform(c_str!("uTexTransform"))?,
            view_state: ViewingState::new(),
            subwindow: Subwindow::new(glam::ivec2(0, 0), glam::vec2(0.0, 0.0), glam::vec2(0.0, 0.0)),
        };
        board.update_window_size(window_dimensions);
        Ok(board)
    }

    pub fn draw(&self, board_data: &gl_obj::TexStorage) {
        self.vao.bind();
        self.draw_texture_prog.bind();
        board_data.bind(gl::TEXTURE1);
        self.transform_unif.set(&self.subwindow.get_transofrm_matrix());
        self.texture_transform_unif.set(&self.view_state.get_texture_transform());
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    pub fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::MouseWheel { y, .. } => {
                self.view_state.zoom(y as f32);
                true
            },
            Event::MouseMotion { mousestate, xrel, yrel , .. } => {
                if mousestate.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                    let dx = xrel as f32 / self.subwindow.get_main_window_size().x as f32
                        / self.subwindow.get_dimensions().x * 2.0
                        * self.view_state.get_scaling_factor();
                    let dy = yrel as f32 / self.subwindow.get_main_window_size().y as f32
                        / self.subwindow.get_dimensions().y * 2.0
                        * self.view_state.get_scaling_factor();
                    self.view_state.mov(glam::vec2(-dx, dy));
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }

    pub fn update_window_size(&mut self, window_dimensions: glam::IVec2) {
        self.subwindow.set_main_window_size(window_dimensions);
        let ratio = window_dimensions.x as f32 / window_dimensions.y as f32;
        if ratio > 1.0 {
            self.subwindow.set_dimensions(glam::vec2(1.0 / ratio, 1.0));
        } else {
            self.subwindow.set_dimensions(glam::vec2(1.0, ratio));
        }
    }

}