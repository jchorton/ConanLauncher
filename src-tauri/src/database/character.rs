
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};

use crate::database;

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {

    pub character_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>

}

#[derive(Deserialize)]
pub struct NewCharacter {

    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>

}

impl Character {

    pub fn fetch_all(conn: &Connection) -> Result<Vec<Character>> {

        let mut stmt = conn.prepare("SELECT CharacterId, Name, Description, Image FROM Characters")?;
        let character_iter = stmt.query_map([], |row| {
            Ok(Character {
                character_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                image: row.get(3)?,
            })
        })?;

        let mut characters = Vec::new();
        for character in character_iter {
            characters.push(character?);
        }

        Ok(characters)
    }

    pub fn insert(conn: &Connection, new_character: NewCharacter) -> Result<()> {

        conn.execute(
            "INSERT INTO Characters (Name, Description, Image) VALUES (?1, ?2, ?3)",
            params![new_character.name, new_character.description, new_character.image],
        )?;
        Ok(())

    }

}

#[tauri::command]
pub fn get_characters() -> Vec<Character> {

    let conn = database::connection().unwrap();
    Character::fetch_all(&conn).unwrap()

}

#[tauri::command]
pub fn add_character(new_character: NewCharacter) {

    let conn = database::connection().unwrap();
    Character::insert(&conn, new_character).unwrap();

}