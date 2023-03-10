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
        // other.len() must equal self.len() !
        for i in 0..self.len() {
            let med_i = self.nth_median(i);
            let ord = self[med_i].cmp(&other[med_i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }
}
