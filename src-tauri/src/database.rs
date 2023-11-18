
use rusqlite::{Connection, Result};
use std::fs;

use crate::utils;

pub mod character;
pub mod character_message;

pub fn connection() -> Result<Connection> {

    let em_dirs = utils::get_em_dirs();
    let path = em_dirs.get_data_dir_path("blinky.db");
    Connection::open(path)

}

pub fn init() -> Result<()> {

    let conn = connection()?;
    let sql = fs::read_to_string("./sql/tables.sql")
        .expect("Failed to read SQL file");

    conn.execute_batch(&sql)?;
    Ok(())

}
