use crate::{gl_obj, resources};

pub struct ProgramRegistry {
    pub draw_texture_prog: gl_obj::Program,
    pub board_step_prog: gl_obj::Program,
}

impl ProgramRegistry {
    pub fn load(shader_resources: &resources::Resources) -> Result<ProgramRegistry, String> {
        Ok(ProgramRegistry {
            draw_texture_prog: gl_obj::Program::from_shaders(&[
                gl_obj::Shader::from_vert_cstr(&shader_resources.load_cstring("board.vert")?)?,
                gl_obj::Shader::from_frag_cstr(&shader_resources.load_cstring("board.frag")?)?
            ])?,
            board_step_prog: gl_obj::Program::from_shaders(&[
                gl_obj::Shader::from_comp_cstr(&shader_resources.load_cstring("board_step.comp")?)?
            ])?
        })
    }
}
