# majority
Rust crate to manage Majority Judgment polls  
https://en.wikipedia.org/wiki/Majority_judgment

```rust
use majority::{Polls, Poll};
use anyhow::Result;

fn poll_demo() -> Result<()> {
	let mut polls = Polls::new("polls.db")?;
	// create a poll
	polls.add_poll(
		"poll 1",
		"user 1",
		"Where shall we eat tomorrow ?",
		vec!["Mama's Pizza", "Mega Sushi", "The Borgir", "Mec Don Hald"],
	)?;

	// user 2 assigns value 3 to "Mama's Pizza"
	polls.vote("poll 1", 0, "user 2", 3)?;
	// ... more votes ...
	// get the poll results !
	poll = polls.get_poll("poll 1")?;
}
```