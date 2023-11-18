
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct CharacterMessage {

    character_message_id: i32,
    create_time: NaiveDateTime,
    character_id: i32,
    message: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
struct NewCharacterMessage {

    character_id: i32,
    message: String,

}

impl NewCharacterMessage {

    fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO CharacterMessages (CharacterId, Message) VALUES (?1, ?2)",
            params![self.character_id, self.message],
        )?;
        Ok(())
    }

}
