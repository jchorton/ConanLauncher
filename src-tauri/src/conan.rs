use std::{fs, process::Command};


pub struct Conan {
    pub working_path: String
}

impl Conan {

    pub fn new(working_path: String) -> Conan {

        Conan {
            working_path: working_path
        }

    }

    fn get_ini_file(&self) -> String {

        match fs::read_to_string(format!("{}/DefaultGame.ini", self.working_path)) {
            Ok(contents) => contents,
            Err(_err) => {
                panic!("Make sure to place this executable next to the ini file!");
            } 
        }

    }

    fn ini_file_not_modified(&self, contents: &str) -> bool {

        contents.contains("+StartupMovies")

    }

    fn modify_file(&self, contents: String) {
        
        match fs::write(format!("{}/DefaultGame-Backup.ini", self.working_path), contents.clone()) {
            Ok(_) => {},
            Err(_) => {}
        }

        let contents = contents.replace("+StartupMovies", "-StartupMovies");

        match fs::write(format!("{}/DefaultGame.ini", self.working_path), contents) {
            Ok(_) => {},
            Err(_) => {}
        }

    }

    pub fn launch_game(&self) {

        let ini_contents = self.get_ini_file();
        if self.ini_file_not_modified(&ini_contents) {
            self.modify_file(ini_contents);
        }

        Command::new(format!("{}/Launcher/FuncomLauncher.exe", self.working_path))
            .spawn()
            .expect("Failed to launch game!");

    }

}