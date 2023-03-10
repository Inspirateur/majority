use crate::votes::MJVotes;
use std::cmp::Ordering;

pub(crate) fn fill_out_votes(options_votes: &mut Vec<Vec<usize>>) {
    // fill out missing votes as 0
    if let Some(participants) = options_votes.iter().map(|votes| votes.len()).max() {
        for votes in options_votes {
            votes.splice(0..0, vec![0; participants - votes.len()]);
        }
    }
}

pub(crate) fn compute_ranking(votes: &Vec<Vec<usize>>) -> Vec<usize> {
    // Given a list of votes (for each option), compute the rank of each option, with ex aequos
    let mut sorted_votes: Vec<_> = votes.clone().into_iter().enumerate().collect();
    sorted_votes.sort_by(|(_i1, v1), (_i2, v2)| v2.vote_cmp(v1));
    let mut res = vec![0; votes.len()];
    res[sorted_votes[0].0] = 1;
    let mut rank = 1;
    for i in 1..votes.len() {
        if sorted_votes[i].1.vote_cmp(&sorted_votes[i - 1].1) != Ordering::Equal {
            rank += 1;
        }
        res[sorted_votes[i].0] = rank;
    }
    res
}
