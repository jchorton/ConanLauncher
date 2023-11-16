use std::{fs, process::{Command, Child}};

use crate::{conan_launch_settings::ConanLaunchSettings, conan_hook::hook_into_process};


pub struct Conan {
    pub ini_dir: String,
    pub launcher_dir: String,
    pub battle_eye_exe: String,
    pub regular_exe: String,
    pub working_path: String,
}

impl Conan {

    pub fn new(working_path: String) -> Conan {

        Conan {
            ini_dir: format!("{}/ConanSandbox/Config", working_path),
            launcher_dir: format!("{}/Launcher", working_path),
            battle_eye_exe: format!("{}/ConanSandbox/Binaries/Win64/ConanSandbox_BE.exe", working_path),
            regular_exe: format!("{}/ConanSandbox/Binaries/Win64/ConanSandbox.exe", working_path),
            working_path
        }

    }

    fn ini_path(&self, relative: &str) -> String {

        format!("{}/{}", self.ini_dir, relative)

    }

    fn launcher_path(&self, relative: &str) -> String {

        format!("{}/{}", self.launcher_dir, relative)

    }

    fn get_ini_file(&self) -> String {

        match fs::read_to_string(self.ini_path("DefaultGame.ini")) {

            Ok(contents) => contents,
            Err(_err) => {
                panic!("Unable to find ini path! {}", self.ini_path("DefaultGame.ini"));
            } 

        }

    }

    fn ini_file_not_modified(&self, contents: &str) -> bool {

        contents.contains("+StartupMovies")

    }

    fn modify_file(&self, contents: String) {
        
        match fs::write(self.ini_path("DefaultGame-Backup.ini"), contents.clone()) {
            Ok(_) => {},
            Err(_) => {}
        }

        let contents = contents.replace("+StartupMovies", "-StartupMovies");

        match fs::write(self.ini_path("DefaultGame.ini"), contents) {
            Ok(_) => {},
            Err(_) => {}
        }

    }

    pub fn launch_game(&self, launch_settings: ConanLaunchSettings) {

        let ini_contents = self.get_ini_file();
        if self.ini_file_not_modified(&ini_contents) {
            self.modify_file(ini_contents);
        }

        let child = if launch_settings.battle_eye {

            Command::new(&self.battle_eye_exe)
                .current_dir(self.working_path.clone())
                .args(self.get_args(&launch_settings))
                .spawn()
                .expect("Failed to launch game!")

        } else {

            Command::new(&self.regular_exe)
                .current_dir(self.working_path.clone())
                .args(self.get_args(&launch_settings))
                .spawn()
                .expect("Failed to launch game!")

        };
        hook_into_process(child);

    }

    fn get_args(&self, launch_settings: &ConanLaunchSettings) -> Vec<&str> {

        let mut starter_settings = if launch_settings.battle_eye {
            vec!["-BattleEye"]
        } else {
            vec![]
        };

        if launch_settings.continue_playing {
            starter_settings.push("-continuesession");
        }

        starter_settings

    }

}