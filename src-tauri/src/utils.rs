
use em_libs::dal::em_dirs::EmDirs;

pub fn get_em_dirs() -> EmDirs {
    EmDirs::new("ConanLauncher")
}