use std::cmp::Ordering;

pub trait MJVotes {
    fn nth_median(&self, n: usize) -> usize;

    fn vote_cmp(&self, other: &Self) -> Ordering;
}

impl MJVotes for Vec<usize> {
    // https://en.wikipedia.org/wiki/Majority_judgment#Voting_process

    fn nth_median(&self, n: usize) -> usize {
        // use div_ceil when stabilized
        let med = (self.len() + 1) / 2 - 1;
        let i = (n + 1) / 2;
        if (self.len() - n) % 2 == 0 {
            med - i
        } else {
            med + i
        }
    }

    fn vote_cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.len().min(other.len()) {
            let self_med= self.nth_median(i);
            let other_med = other.nth_median(i);
            let ord = self[self_med].cmp(&other[other_med]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }
}
