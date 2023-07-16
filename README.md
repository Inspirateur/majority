# majority
Rust crate to manage Majority Judgment polls  
https://electowiki.org/wiki/Majority_Judgment

```rust
use majority::{Polls, Poll};
use anyhow::Result;

fn poll_demo() -> Result<()> {
	// variables for readability
	let poll_id: u64 = 1;
	let user_1: u64 = 1;
	let user_2: u64 = 2;
	let mut polls = Polls::new("polls.db")?;
	// create a poll
	polls.add_poll(
		poll_id,
		user_1,
		"Where shall we eat tomorrow ?",
		vec!["Mama's Pizza", "Mega Sushi", "The Borgir", "Mec Don Hald"],
	)?;

	// user 2 assigns value 3 to "Mama's Pizza" (option 0)
	polls.vote(poll_id, 0, user_2, 3)?;
	// ... more votes ...
	// get the poll results !
	poll = polls.get_poll(poll_id)?;
}
```