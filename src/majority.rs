use std::collections::HashMap;

use anyhow::Result;
use rusqlite::Connection;

pub struct Poll {
    desc: String,
    author: String,
    options: Vec<String>,
    votes: Vec<HashMap<usize, usize>>,
    ranking: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Polls {
    db_path: String,
}

impl Polls {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Poll (
                uuid TEXT PRIMARY KEY,
				desc TEXT,
				author TEXT,
                is_open INTEGER DEFAULT 1
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Options (
				poll TEXT REFERENCES Poll(uuid) ON DELETE CASCADE,
				number INTEGER,
				desc TEXT,
				PRIMARY KEY(poll, number)
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Votes (
				user TEXT,
				poll TEXT,
				number INTEGER,
				vote INTEGER,
				PRIMARY KEY(user, poll, number),
				FOREIGN KEY(poll, number) REFERENCES Options(poll, number) ON DELETE CASCADE
            )",
            [],
        )?;
        conn.execute(
            "DELETE FROM Poll
            WHERE is_open = 0",
            [],
        )?;
        Ok(Polls {
            db_path: db_path.to_string(),
        })
    }

    pub fn add_poll(
        &self,
        poll_uuid: String,
        author_uuid: String,
        options: Vec<String>,
    ) -> Result<()> {
        todo!()
    }

    pub fn vote(
        &self,
        poll_uuid: String,
        option_number: usize,
        user_uuid: String,
        value: usize,
    ) -> Result<Poll> {
        todo!()
    }

    pub fn get_poll(&self, poll_uuid: String) -> Result<Poll> {
        todo!()
    }

    pub fn close_poll(&self, poll_uuid: String) -> Result<Poll> {
        todo!()
    }
}
