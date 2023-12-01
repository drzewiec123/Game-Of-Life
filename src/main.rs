#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gl;
extern crate sdl2;
mod gl_obj;
mod utils;
mod resources;
mod board_display;
mod board_state;
mod registry;

use std::{iter, path::Path};
use board_display::BoardDisplay;
use board_state::BoardState;
use registry::Registry;
use sdl2::{event::{Event, WindowEvent}, mouse::MouseButton};
use utils::print_errors;

fn main() {
    let context = utils::init_sdl_window();
    let mut event_pump: sdl2::EventPump = context.sdl.event_pump().unwrap();

    context.set_viewport(glam::ivec2(900, 700));
    context.set_clear_color(glam::vec4(0.0, 0.0, 0.0, 1.0));

    let resources = resources::Resources::from_relative_exe_path(Path::new("assets/shaders")).unwrap();
    let registry = Registry::load(&resources).unwrap();
    let mut display = BoardDisplay::new(&registry.program_registry, glam::ivec2(900, 700)).unwrap();
    let data : &[gl::types::GLubyte] = &[
        1, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 1,
    ];
    let mut board_state = BoardState::new(&registry.program_registry, 8, 8, data).unwrap();

    'mainloop: loop {
        context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        display.draw(board_state.get_current_board());
        context.window.gl_swap_window();

        let mut update = false;
        while !update {
            update = false;
            for event in iter::once(event_pump.wait_event()).chain(event_pump.poll_iter()) {
                let mut redrawing_event_occured = true;
                match event {
                    Event::Quit {..} => break 'mainloop,
                    Event::Window { win_event, ..} => {
                        match win_event {
                            WindowEvent::SizeChanged(w, h) => {
                                context.set_viewport(glam::ivec2(w, h));
                                display.update_window_size(glam::ivec2(w, h));
                            },
                            _ => redrawing_event_occured = false,
                        }
                    },
                    Event::MouseButtonDown { mouse_btn, ..} => {
                        match mouse_btn {
                            MouseButton::Right => board_state.step(),
                            _ => redrawing_event_occured = false
                        }
                    }
                    _ => redrawing_event_occured = display.handle_event(event),
                }
                update = update || redrawing_event_occured;
            }
        }

    }
    print_errors("exit");
}
