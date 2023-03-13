mod db;
mod majority_judgment;
mod poll;
mod votes;
pub use db::Polls;
pub use poll::Poll;

#[cfg(test)]
mod tests {
    use crate::{Poll, *};
    use anyhow::{Ok, Result};
    
    fn is_sync<T: Sync>() { }

    fn votes_on(polls: &Polls, poll_uuid: &str, user_votes: Vec<Vec<usize>>) -> Result<Poll> {
        for (user, votes) in user_votes.into_iter().enumerate() {
            let user_uuid = (user + 1).to_string();
            for (opt, vote) in votes.into_iter().enumerate() {
                polls.vote(poll_uuid, opt, &user_uuid, vote)?;
            }
        }
        Ok(polls.get_poll(poll_uuid)?)
    }

    #[test]
    fn it_works() {
        // To ensure that Polls is Sync
        is_sync::<Polls>();
        let polls = Polls::new("polls.db").unwrap();
        // Create a poll with 3 options
        polls
            .add_poll(
                "1",
                "1",
                "Where shall we eat tomorrow ?",
                vec!["Mama's Pizza", "Mega Sushi", "The Borgir"],
            )
            .unwrap();
        // Add a fourth one
        polls.add_options("1", vec!["Mec Don Hald"]).unwrap();
        // Vote values between 1-5 for this test
        let user_votes = vec![
            //   0  1  2  3  options
            vec![3, 5, 3, 1], // user 1
            vec![4, 2, 4, 5], // user 2
            vec![3, 3, 3, 3], // user 3
            vec![5, 3, 4, 2], // user 4
            vec![5, 5, 5, 2], // user 5
            vec![2, 1, 2, 4], // user 6
        ];
        let poll = votes_on(&polls, "1", user_votes).unwrap();
        print!("{}", poll);
        assert_eq!(poll.ranking, vec![1, 3, 2, 4])
    }
}
