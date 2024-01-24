use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};

use std::fs;

use lettre::{message::header::ContentType, transport::smtp::{authentication::Mechanism, client::Tls}};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Deserialize, Serialize, Debug)]
pub struct TextingInfo {
    pub email_relay: String,
    pub user_email: String,
    pub user_password: String,
    pub phone_number: String,
    pub phone_provider: String
}

impl TextingInfo {

    pub fn from_file(passphrase: String) -> Option<TextingInfo> {

        let dirs = utils::get_em_dirs();
        match fs::read(dirs.get_data_dir_path("texting.age")) {

            Ok(contents) => {

                match utils::decrypt_age_content(contents, passphrase) {

                    Ok(content) => {
                        return Some(serde_json::from_str(&content).unwrap());
                    },
                    Err(_) => {}

                }

            },
            Err(_) => {}

        }
        return None

    }

    pub fn save(&self, passphrase: String) {

        let dirs = utils::get_em_dirs();
        match utils::encrypt_age_content(serde_json::to_string(self).unwrap(), passphrase) {
            Ok(encrypted) => {
                fs::write(dirs.get_data_dir_path("texting.age"), encrypted).unwrap();
            },
            Err(_) => {}
        }

    }

}


lazy_static! {
    static ref TEXT_USER: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref USER_TEXTING_INFO: Arc<Mutex<Option<TextingInfo>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
pub fn set_text_user(text_user: bool) {
    TEXT_USER.store(text_user, Ordering::Relaxed);
}

pub fn send_text(message: String) {

    if !TEXT_USER.load(Ordering::Relaxed) {
        return;
    }

    let user_texting_info = USER_TEXTING_INFO.lock().unwrap();
    if let Some(texting_info) = user_texting_info.as_ref() {

        let message: Message = Message::builder()
            .from(format!("Conan Launcher <{}>", texting_info.user_email).parse().unwrap())
            .to(format!("{}@{}", texting_info.phone_number, texting_info.phone_provider).parse().unwrap())
            .subject("Someone is nearby")
            .header(ContentType::TEXT_PLAIN)
            .body(message).unwrap();

        let sender = SmtpTransport::starttls_relay(&texting_info.email_relay).unwrap()
            .credentials(Credentials::new(texting_info.user_email.clone(), texting_info.user_password.clone()))
            .authentication(vec![Mechanism::Plain])
            .build();

        sender.send(&message).unwrap();

    }

}