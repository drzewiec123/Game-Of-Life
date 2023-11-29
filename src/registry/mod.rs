use crate::resources;

pub mod program_registry;

pub struct Registry {
    pub program_registry: program_registry::ProgramRegistry
}

impl Registry {
    pub fn load(resources: &resources::Resources) -> Result<Registry, String> {
        Ok(Registry {
            program_registry: program_registry::ProgramRegistry::load(resources)?
        })
    }
}
