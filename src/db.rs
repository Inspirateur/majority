use crate::{
    majority_judgment::{compute_ranking, fill_out_votes},
    Poll,
};
use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::path::Path;

#[derive(Debug)]
pub struct Polls {
    conn: Connection,
}

impl Polls {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
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
            "CREATE TABLE IF NOT EXISTS Vote (
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
        Ok(Polls { conn })
    }

    pub fn add_poll<S: Into<String>>(
        &mut self,
        poll_uuid: S,
        author_uuid: S,
        desc: S,
        options: Vec<S>,
    ) -> Result<()> {
        let poll_uuid = poll_uuid.into();
        let author_uuid = author_uuid.into();
        let desc = desc.into();
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT 
            INTO Poll (uuid, author, desc) 
            VALUES (?1, ?2, ?3)",
            [poll_uuid.clone(), author_uuid, desc],
        )?;
        for (i, opt_desc) in options.into_iter().enumerate() {
            tx.execute(
                "INSERT 
                INTO Option (poll, number, desc) 
                VALUES (?1, ?2, ?3)",
                [poll_uuid.clone(), i.to_string(), opt_desc.into()],
            )?;
        }
        Ok(tx.commit()?)
    }

    pub fn add_option<S: Into<String>>(&self, poll_uuid: S, option: S) -> Result<()> {
        let poll_uuid = poll_uuid.into();
        let count = self
            .conn
            .prepare("SELECT COUNT(*) FROM Option WHERE poll = ?1")
            .unwrap()
            .query_row([poll_uuid.clone()], |row| row.get::<usize, usize>(0))?;
        self.conn.execute(
            "INSERT
            INTO Option (poll, number, desc)
            VALUES (?1, ?2, ?3)",
            [poll_uuid, count.to_string(), option.into()],
        )?;
        Ok(())
    }

    pub fn vote<S: Into<String>>(
        &self,
        poll_uuid: S,
        option_number: usize,
        user_uuid: S,
        value: usize,
    ) -> Result<Poll> {
        let poll_uuid = poll_uuid.into();
        let user_uuid = user_uuid.into();
        // check if the poll is open
        if !self.is_poll_open(poll_uuid.clone())? {
            return Err(anyhow!("Poll is closed"));
        }
        self.conn.execute(
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

    pub fn get_poll<S: Into<String>>(&self, poll_uuid: S) -> Result<Poll> {
        let poll_uuid = poll_uuid.into();
        let (author, desc, is_open) = self
            .conn
            .prepare("SELECT author, desc, is_open FROM Poll WHERE uuid = ?1")
            .unwrap()
            .query_row([poll_uuid.clone()], |row| {
                Ok((
                    row.get::<usize, String>(0)?,
                    row.get::<usize, String>(1)?,
                    row.get::<usize, bool>(2)?,
                ))
            })?;
        let mut stmt = self
            .conn
            .prepare("SELECT desc FROM Option WHERE poll = ?1 ORDER BY number ASC")
            .unwrap();
        let options = stmt
            .query_map([poll_uuid.clone()], |row| row.get::<usize, String>(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        let mut stmt = self
            .conn
            .prepare("SELECT number, vote FROM Vote WHERE poll = ?1")
            .unwrap();
        let _votes = stmt
            .query_map([poll_uuid], |row| {
                Ok((row.get::<usize, usize>(0)?, row.get::<usize, usize>(1)?))
            })?
            .collect::<Result<Vec<(usize, usize)>, rusqlite::Error>>()?;
        let mut options_votes: Vec<Vec<usize>> = (0..options.len()).map(|_i| Vec::new()).collect();
        for (number, vote) in _votes {
            options_votes[number].push(vote);
        }
        for votes in options_votes.iter_mut() {
            votes.sort()
        }
        fill_out_votes(&mut options_votes);
        Ok(Poll {
            desc,
            author,
            is_open,
            options,
            ranking: compute_ranking(&options_votes),
            votes: options_votes,
        })
    }

    pub fn is_poll_open<S: Into<String>>(&self, poll_uuid: S) -> Result<bool> {
        Ok(self
            .conn
            .prepare("SELECT is_open FROM Poll WHERE uuid = ?1")
            .unwrap()
            .query_row([poll_uuid.into()], |row| row.get::<usize, bool>(0))?)
    }

    pub fn close_poll<S: Into<String>>(&self, poll_uuid: S) -> Result<Poll> {
        let poll_uuid = poll_uuid.into();
        self.conn.execute(
            "UPDATE Poll
            SET is_open = 0
            WHERE uuid = ?1",
            [poll_uuid.clone()],
        )?;
        self.get_poll(poll_uuid)
    }
}
