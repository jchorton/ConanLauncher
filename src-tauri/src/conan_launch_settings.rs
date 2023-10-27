
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConanLaunchSettings {
    pub continue_playing: bool,
    pub battle_eye: bool
}