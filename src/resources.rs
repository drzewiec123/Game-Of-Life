use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use std::ffi;

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, String> {
        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| "Failed to get exe path")?;

        let exe_path = exe_file_name.parent()
            .ok_or("Failed to get exe path")?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, String> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
        ).map_err(|err| err.to_string())?;
        
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata().map_err(|err| err.to_string())?.len() as usize + 1
        );
        file.read_to_end(&mut buffer).map_err(|err| err.to_string())?;
        
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err("File contains nil".to_owned());
        }
        
        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
