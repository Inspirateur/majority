use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::collections::HashMap;

use crate::majority_judgment::compute_ranking;

pub struct Poll {
    desc: String,
    author: String,
    options: Vec<String>,
    votes: Vec<HashMap<usize, usize>>,
    ranking: Vec<usize>,
    is_open: bool,
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
            "CREATE TABLE IF NOT EXISTS Option (
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
				FOREIGN KEY(poll, number) REFERENCES Option(poll, number) ON DELETE CASCADE
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
        desc: String,
        options: Vec<String>,
    ) -> Result<()> {
        let mut conn = Connection::open(&self.db_path)?;
        let tx = conn.transaction()?;
        tx.execute(
            "INSERT 
            INTO Poll (uuid, author, desc) 
            VALUES (?1, ?2, ?3)",
            [poll_uuid.clone(), author_uuid, desc],
        )?;
        for (i, desc) in options.into_iter().enumerate() {
            tx.execute(
                "INSERT 
                INTO Option (poll, number, desc) 
                VALUES (?1, ?2, ?3, ?4)",
                [poll_uuid.clone(), i.to_string(), desc],
            )?;
        }
        Ok(tx.commit()?)
    }

    pub fn vote(
        &self,
        poll_uuid: String,
        option_number: usize,
        user_uuid: String,
        value: usize,
    ) -> Result<Poll> {
        let conn = Connection::open(&self.db_path)?;
        // check if the poll is open
        if !Polls::_is_poll_open(&conn, poll_uuid.clone())? {
            return Err(anyhow!("Poll is closed"));
        }
        conn.execute(
            "INSERT OR REPLACE 
            INTO Vote (user, poll, number, vote) 
            VALUES (?1, ?2, ?3, ?4)",
            [
                user_uuid,
                poll_uuid.clone(),
                option_number.to_string(),
                value.to_string(),
            ],
        )?;
        self.get_poll(poll_uuid)
    }

    pub fn get_poll(&self, poll_uuid: String) -> Result<Poll> {
        let conn = Connection::open(&self.db_path)?;
        let (author, desc, is_open) = conn
            .prepare("SELECT author, desc, is_open FROM Poll WHERE uuid = ?1")
            .unwrap()
            .query_row([poll_uuid.clone()], |row| {
                Ok((
                    row.get::<usize, String>(0)?,
                    row.get::<usize, String>(1)?,
                    row.get::<usize, bool>(2)?,
                ))
            })?;
        let mut stmt = conn
            .prepare("SELECT desc FROM Option WHERE poll = ?1 ORDER BY number ASC")
            .unwrap();
        let options = stmt
            .query_map([poll_uuid.clone()], |row| row.get::<usize, String>(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        let mut stmt = conn
            .prepare("SELECT number vote FROM Vote WHERE poll = ?1")
            .unwrap();
        let _votes = stmt
            .query_map([poll_uuid], |row| {
                Ok((row.get::<usize, usize>(0)?, row.get::<usize, usize>(1)?))
            })?
            .collect::<Result<Vec<(usize, usize)>, rusqlite::Error>>()?;
        let mut votes: Vec<HashMap<usize, usize>> =
            (0..options.len()).map(|_i| HashMap::new()).collect();
        for (number, vote) in _votes {
            votes[number]
                .entry(vote)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        Ok(Poll {
            desc,
            author,
            is_open,
            options,
            ranking: compute_ranking(&votes),
            votes,
        })
    }

    fn _is_poll_open(conn: &Connection, poll_uuid: String) -> Result<bool> {
        Ok(conn
            .prepare("SELECT is_open FROM Poll WHERE uuid = ?1")
            .unwrap()
            .query_row([poll_uuid], |row| row.get::<usize, bool>(0))?)
    }

    pub fn is_poll_open(&self, poll_uuid: String) -> Result<bool> {
        let conn = Connection::open(&self.db_path)?;
        Polls::_is_poll_open(&conn, poll_uuid)
    }

    pub fn close_poll(&self, poll_uuid: String) -> Result<Poll> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "UPDATE Poll
            SET is_open = 0
            WHERE uuid = ?1",
            [poll_uuid.clone()],
        )?;
        self.get_poll(poll_uuid)
    }
}
