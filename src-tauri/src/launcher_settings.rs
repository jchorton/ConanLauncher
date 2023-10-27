
use std::{fs, path::Path};

use em_libs::dal::em_dirs::EmDirs;
use serde::{Deserialize, Serialize};

use crate::conan::Conan;

#[derive(Deserialize, Serialize)]
pub struct LauncherSettings {
    pub path: String,
    pub battle_eye: bool
}

impl LauncherSettings {

    pub fn new(path: String) -> LauncherSettings {

        LauncherSettings {
            path,
            battle_eye: false
        }

    }

    pub fn from_file() -> Option<LauncherSettings> {

        let dirs = EmDirs::new("ConanLauncher");
        match fs::read_to_string(dirs.get_data_dir_path("settings.json")) {

            Ok(contents) => {
                let install_dir: LauncherSettings = serde_json::from_str(&contents).unwrap();
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
        fs::write(dirs.get_data_dir_path("settings.json"), serde_json::to_string(&self).unwrap()).unwrap();

    }

    pub fn into_conan(self) -> Conan {
            
        Conan::new(self.path)

    }

}