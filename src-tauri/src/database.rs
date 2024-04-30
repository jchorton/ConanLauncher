
use rusqlite::{Connection, Result};
use std::fs;

use crate::utils;

pub mod character;
pub mod message;
pub mod character_message;

const TABLES: &str = include_str!("../sql/tables.sql");

pub fn connection() -> Result<Connection> {

    let em_dirs = utils::get_em_dirs();
    let path = em_dirs.get_data_dir_path("blinky.db");
    Connection::open(path)

}

pub fn init() -> Result<()> {

    let conn = connection()?;
    conn.execute_batch(TABLES)?;
    Ok(())

}
