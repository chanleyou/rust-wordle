mod game;
mod words;
use std::io;

pub fn main() {
	println!("Welcome to Wordle!");

	game::play();

	loop {
		println!("\nPlay again? (Y/N)");

		let mut response = String::new();

		io::stdin()
			.read_line(&mut response)
			.expect("Failed to read line.");

		let response = response.trim().to_lowercase();

		match response.as_str() {
			"y" => {}
			"n" => break,
			_ => {
				println!("Invalid response.");
				continue;
			}
		}

		game::play();
	}
}
