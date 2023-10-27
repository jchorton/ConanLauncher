use std::{fs, process::Command};


pub struct Conan {
    pub ini_dir: String,
    pub launcher_dir: String,
    pub working_path: String
}

impl Conan {

    pub fn new(working_path: String) -> Conan {

        Conan {
            ini_dir: format!("{}/ConanSandbox/Config", working_path),
            launcher_dir: format!("{}/Launcher", working_path),
            working_path: working_path
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

    pub fn launch_game(&self) {

        let ini_contents = self.get_ini_file();
        if self.ini_file_not_modified(&ini_contents) {
            self.modify_file(ini_contents);
        }

        Command::new(self.launcher_path("FuncomLauncher.exe"))
            .spawn()
            .expect("Failed to launch game!");

    }

}