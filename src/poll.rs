use std::fmt::Display;

pub struct Poll {
    pub desc: String,
    pub author: String,
    pub options: Vec<String>,
    pub votes: Vec<Vec<usize>>,
    pub ranking: Vec<usize>,
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
