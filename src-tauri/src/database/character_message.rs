
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::database;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMessage {

    pub character_message_id: i32,
    pub create_time: NaiveDateTime,
    pub character_id: i32,
    pub message: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCharacterMessage {

    pub character_id: i32,
    pub message: String,

}

impl NewCharacterMessage {

    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO CharacterMessages (CharacterId, Message) VALUES (?1, ?2)",
            params![self.character_id, self.message],
        )?;
        Ok(())
    }

    pub fn insert_new_db(&self) -> Result<()> {

        let conn = database::connection()?;
        self.insert(&conn)
        
    }

}
