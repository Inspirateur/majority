use std::fmt::Display;
use anyhow::{Error, anyhow};

#[derive(PartialEq)]
pub enum DefaultVote {
    // Unexpressed votes default to lowest rating
    REJECT = 0,
    // Unexpressed votes are ignored
    IGNORE = 1
}

impl TryFrom<u32> for DefaultVote {
    type Error = Error;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == DefaultVote::REJECT as u32 => Ok(DefaultVote::REJECT),
            x if x == DefaultVote::IGNORE as u32 => Ok(DefaultVote::IGNORE),
            _ => Err(anyhow!("Unknown DefaultVote mode {}", v)),
        }
    }
}

pub struct Poll {
    pub desc: String,
    pub author: String,
    pub options: Vec<String>,
    pub votes: Vec<Vec<usize>>,
    pub ranking: Vec<usize>,
    pub default_vote: DefaultVote,
    pub is_open: bool,
}

impl Display for Poll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(poll from {})\n{}\n{}",
            self.author,
            self.desc,
            self.options
                .iter()
                .zip(&self.votes)
                .zip(&self.ranking)
                .map(|((opt_desc, votes), rank)| format!(
                    "({}) {}\n{}\n",
                    rank,
                    opt_desc,
                    votes
                        .into_iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
                .fold(String::new(), |a, b| a + &b + "\n")
        )
    }
}
