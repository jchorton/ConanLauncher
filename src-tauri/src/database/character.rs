
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};

use crate::database;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub fn insert(conn: &Connection, new_character: NewCharacter) -> Result<Character> {

        conn.execute(
            "INSERT INTO Characters (Name, Description, Image) VALUES (?1, ?2, ?3)",
            params![new_character.name, new_character.description, new_character.image],
        )?;
        Ok(Self::fetch_new(conn))

    }

    fn fetch_new(conn: &Connection) -> Character {

        let mut stmt = conn.prepare("SELECT CharacterId, Name, Description, Image FROM Characters ORDER BY CharacterId DESC LIMIT 1").unwrap();
        stmt.query_row([], |row| {
            Ok(Character {
                character_id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                image: row.get(3).unwrap(),
            })
        }).unwrap()

    }

}

#[tauri::command]
pub fn get_characters() -> Vec<Character> {

    let conn = database::connection().unwrap();
    Character::fetch_all(&conn).unwrap()

}

#[tauri::command]
pub fn add_character(window: tauri::Window, new_character: NewCharacter) {

    let conn = database::connection().unwrap();
    let character = Character::insert(&conn, new_character).unwrap();

    window.emit("character_added", character).unwrap();

}