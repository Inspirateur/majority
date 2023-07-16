use crate::{
    majority_judgment::{compute_ranking, fill_out_votes},
    Poll, poll::DefaultVote,
};
use anyhow::{anyhow, Result};
use rusqlite::{Connection, params};
use std::{path::{Path, PathBuf}};

pub struct Polls {
    path: PathBuf,
}

impl Polls {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let path = db_path.as_ref().to_path_buf();
        let conn = Connection::open(&path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Poll (
                uuid INTEGER PRIMARY KEY,
				desc TEXT,
				author INTEGER,
                default_mode INGEGER,
                is_open INTEGER DEFAULT 1
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Option (
				poll INTEGER REFERENCES Poll(uuid) ON DELETE CASCADE,
				number INTEGER,
				desc TEXT,
				PRIMARY KEY(poll, number)
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Vote (
				user INTEGER,
				poll INTEGER,
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
        Ok(Polls { path })
    }

    pub fn add_poll<S1, S2, S3, S4>(
        &self,
        poll_uuid: S1,
        author_uuid: S2,
        desc: S3,
        options: Vec<S4>,
        default_vote: DefaultVote
    ) -> Result<Poll> 
    where S1: Into<u64>, S2: Into<u64>, S3: ToString, S4: ToString {
        let conn = Connection::open(&self.path)?;
        let poll_uuid = poll_uuid.into();
        let author_uuid = author_uuid.into();
        let desc = desc.to_string();
        conn.execute(
            "INSERT 
            INTO Poll (uuid, author, desc, default_mode) 
            VALUES (?1, ?2, ?3, ?4)",
            params![poll_uuid.clone(), author_uuid, desc, default_vote as u32],
        )?;
        for (i, opt_desc) in options.into_iter().enumerate() {
            conn.execute(
                "INSERT 
                INTO Option (poll, number, desc) 
                VALUES (?1, ?2, ?3)",
                params![poll_uuid.clone(), i as u64, opt_desc.to_string()],
            )?;
        }
        Polls::_get_poll(&conn, poll_uuid)
    }

    pub fn add_options<S1, S2>(&self, poll_uuid: S1, options: Vec<S2>) -> Result<Poll>
    where S1: Into<u64>, S2: ToString {
        let conn = Connection::open(&self.path)?;
        let poll_uuid = poll_uuid.into();
        let count = conn
            .prepare("SELECT COUNT(*) FROM Option WHERE poll = ?1")
            .unwrap()
            .query_row([poll_uuid.clone()], |row| row.get::<usize, usize>(0))?;
        for (i, option) in options.into_iter().enumerate() {
            conn.execute(
                "INSERT
                INTO Option (poll, number, desc)
                VALUES (?1, ?2, ?3)",
                params![poll_uuid.clone(), (count + i) as u64, option.to_string()],
            )?;
        }
        Polls::_get_poll(&conn, poll_uuid)
    }

    pub fn vote<S1, S2>(
        &self,
        poll_uuid: S1,
        option_number: usize,
        user_uuid: S2,
        value: usize,
    ) -> Result<Poll> 
    where S1: Into<u64>, S2: Into<u64> {
        let conn = Connection::open(&self.path)?;
        let poll_uuid = poll_uuid.into();
        let user_uuid = user_uuid.into();
        // check if the poll is open
        if !self.is_poll_open(poll_uuid.clone())? {
            return Err(anyhow!("Poll is closed"));
        }
        conn.execute(
            "INSERT OR REPLACE 
            INTO Vote (user, poll, number, vote) 
            VALUES (?1, ?2, ?3, ?4)",
            [
                user_uuid,
                poll_uuid.clone(),
                option_number as u64,
                value as u64,
            ],
        )?;
        Polls::_get_poll(&conn, poll_uuid)
    }

    fn _get_poll<S: Into<u64>>(conn: &Connection, poll_uuid: S) -> Result<Poll> {
        let poll_uuid = poll_uuid.into();
        let (author, desc, default_vote_res, is_open) = conn
            .prepare("SELECT author, desc, default_mode, is_open FROM Poll WHERE uuid = ?1")
            .unwrap()
            .query_row([poll_uuid.clone()], |row| {
                Ok((
                    row.get::<usize, u64>(0)?,
                    row.get::<usize, String>(1)?,
                    DefaultVote::try_from(row.get::<usize, u32>(2)?),
                    row.get::<usize, bool>(3)?,
                ))
            })?;
        let default_vote = default_vote_res?;
        let mut stmt = conn
            .prepare("SELECT desc FROM Option WHERE poll = ?1 ORDER BY number ASC")
            .unwrap();
        let options = stmt
            .query_map([poll_uuid.clone()], |row| row.get::<usize, String>(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;
        let mut stmt = conn
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
        if default_vote == DefaultVote::REJECT {
            fill_out_votes(&mut options_votes);
        }
        Ok(Poll {
            desc,
            author,
            is_open,
            options,
            ranking: compute_ranking(&options_votes),
            default_vote,
            votes: options_votes,
        })
    }

    pub fn get_poll<S: Into<u64>>(&self, poll_uuid: S) -> Result<Poll> {
        let conn = Connection::open(&self.path)?;
        Polls::_get_poll(&conn, poll_uuid)
    }

    pub fn is_poll_open<S: Into<u64>>(&self, poll_uuid: S) -> Result<bool> {
        let conn = Connection::open(&self.path)?;
        let res = conn
            .prepare("SELECT is_open FROM Poll WHERE uuid = ?1")
            .unwrap()
            .query_row([poll_uuid.into()], |row| row.get::<usize, bool>(0))?;
        Ok(res)
    }

    pub fn close_poll<S: Into<u64>>(&self, poll_uuid: S) -> Result<Poll> {
        let conn = Connection::open(&self.path)?;
        let poll_uuid = poll_uuid.into();
        conn.execute(
            "UPDATE Poll
            SET is_open = 0
            WHERE uuid = ?1",
            [poll_uuid.clone()],
        )?;
        Polls::_get_poll(&conn, poll_uuid)
    }
}
