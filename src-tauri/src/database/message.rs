
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {

    message_id: i32,
    create_time: NaiveDateTime,
    sender: String,
    message: String,

}

#[derive(Debug, Serialize, Deserialize, FromForm, Clone)]
pub struct NewMessage {

    pub message: String,
    pub sender: String,

}

impl Message {

    pub fn fetch_all(conn: &Connection) -> Result<Vec<Message>> {

        let mut stmt = conn.prepare("SELECT MessageId, CreateTime, Sender, Message FROM Messages")?;
        let messages = stmt.query_map([], |row| {
            Ok(Message {
                message_id: row.get(0)?,
                create_time: row.get(1)?,
                sender: row.get(2)?,
                message: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<Message>>>()?;

        Ok(messages)

    }

    pub fn insert(conn: &Connection, new_message: NewMessage) -> Result<()> {

        conn.execute(
            "INSERT INTO Messages (Sender, Message) VALUES (?1, ?2)",
            params![new_message.sender, new_message.message],
        )?;
        Ok(())

    }

    pub fn batch_insert(conn: &mut Connection, new_messages: Vec<NewMessage>) -> Result<()> {

        let tx = conn.transaction()?;
        for new_message in new_messages {
            tx.execute(
                "INSERT INTO Messages (Sender, Message) VALUES (?1, ?2)",
                params![new_message.sender, new_message.message],
            )?;
        }
        tx.commit()?;
        Ok(())

    }

}
