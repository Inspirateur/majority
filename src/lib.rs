mod db;
mod majority_judgment;
pub use db::Polls;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let polls = Polls::new("polls.db").unwrap();
    }
}
