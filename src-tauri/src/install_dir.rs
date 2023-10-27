
use std::{fs, path::Path};

use em_libs::dal::em_dirs::EmDirs;
use serde::{Deserialize, Serialize};

use crate::conan::Conan;

#[derive(Deserialize, Serialize)]
pub struct InstallDir {
    pub path: String
}

impl InstallDir {

    pub fn new(path: String) -> InstallDir {

        InstallDir {
            path
        }

    }

    pub fn from_file() -> Option<InstallDir> {

        let dirs = EmDirs::new("ConanLauncher");
        match fs::read_to_string(dirs.get_data_dir_path("install_location.json")) {

            Ok(contents) => {
                let install_dir: InstallDir = serde_json::from_str(&contents).unwrap();
                return Some(install_dir);
            },
            Err(_) =>{
                return None;
            }

        }

    }

    pub fn valid(&self) -> bool {

        if !Path::new(format!("{}/ConanSandbox", self.path).as_str()).exists() {
            return false;
        }

        if !Path::new(format!("{}/Launcher", self.path).as_str()).exists() {
            return false;
        }

        return true;

    }

    pub fn save(&self) {

        let dirs = EmDirs::new("ConanLauncher");
        fs::write(dirs.get_data_dir_path("install_location.json"), serde_json::to_string(&self).unwrap()).unwrap();

    }

    pub fn into_conan(self) -> Conan {
            
        Conan::new(self.path)

    }

}