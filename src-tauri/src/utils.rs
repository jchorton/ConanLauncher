
use std::io::{Read, Write};
use age::secrecy::Secret;

use em_libs::dal::em_dirs::EmDirs;

pub fn get_em_dirs() -> EmDirs {
    EmDirs::new("ConanLauncher")
}

pub fn decrypt_age_content(content: Vec<u8>, passphrase: String) -> Result<String, &'static str> {

    let decryptor = match age::Decryptor::new(content.as_slice()) {
        Ok(decryptor) => {
            match decryptor {
               age::Decryptor::Passphrase(d) => d,
               _ => unreachable!() 
            }
        },
        Err(_) => {
            return Err("Failed to decrypt content");
        }
    };

    let mut decrypted: Vec<u8> = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(passphrase), None).map_err(|_| "Failed to decrypt content with passphrase")?;
    reader.read_to_end(&mut decrypted).map_err(|_| "Reader failed to decrypt content.")?;
    Ok(String::from_utf8(decrypted).unwrap())

}

pub fn encrypt_age_content(content: String, passphrase: String) -> Result<Vec<u8>, &'static str> {

    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase));
    let mut encrypted: Vec<u8> = vec![];

    let mut writer = encryptor.wrap_output(&mut encrypted).map_err(|_| "Failed to encrypt content with passphrase")?;
    writer.write_all(content.as_bytes()).map_err(|_| "Writer failed to encrypt content.")?;
    writer.finish().unwrap();

    Ok(encrypted)

}