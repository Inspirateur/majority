use std::cmp::Ordering;

pub trait MJVotes {
    fn nth_median(&self, n: usize) -> usize;

    fn vote_cmp(&self, other: &Self) -> Ordering;
}

impl MJVotes for Vec<usize> {
    /// Return the "nth" median of the votes. 
    /// The 0th median is the median in the traditionnal sense, 
    /// the other are neighboring medians used for tie-breaking as defined by the Majority Judgement 
    /// https://en.wikipedia.org/wiki/Majority_judgment#Voting_process
    fn nth_median(&self, n: usize) -> usize {
        if self.len() == 0 {
            return 0;
        }
        // TODO: use div_ceil when stabilized
        let med = (self.len() + 1) / 2 - 1;
        let i = (n + 1) / 2;
        if (self.len() - n) % 2 == 0 {
            self[med - i]
        } else {
            self[med + i]
        }
    }

    /// Orders a pair of votes vector, using nth_median(i) iteratively until a winner is established
    fn vote_cmp(&self, other: &Self) -> Ordering {
        // the 0th median is defined as 0 even if there is NO votes
        // this is why we evaluate at least the 0th median when there is no votes 
        for i in 0..self.len().min(other.len()).max(1) {
            let self_med= self.nth_median(i);
            let other_med = other.nth_median(i);
            let ord = self_med.cmp(&other_med);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }
}
