mod majority;
pub use majority::Polls;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let polls = Polls::new("polls.db").unwrap();
    }
}
