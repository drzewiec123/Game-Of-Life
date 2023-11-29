use std::ffi::CString;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct MainContext {
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
}

impl MainContext {
    pub fn clear(&self, params: u32) {
        unsafe {
            gl::Clear(params);
        }
    }

    pub fn set_clear_color(&self, color: glam::Vec4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    pub fn set_viewport(&self, dims: glam::IVec2) {
        unsafe {
            gl::Viewport(0, 0, dims.x, dims.y);
        }
    }
}

pub fn init_sdl_window() -> MainContext {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Hello Window", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    MainContext {
        sdl, window, gl_context
    }
}

pub fn print_errors(label: &str) {
    unsafe {
        loop {
            let err = gl::GetError();
            if err == gl::NO_ERROR {
                break;
            }
            println!("{} Error: {} {:#04x}", label, err, err);
        }
    }
}
