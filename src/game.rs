use crate::words;
use std::collections::HashMap;
use std::error::Error;
use std::io;

pub fn play() {
	if let Err(e) = game() {
		panic!("{}", e);
	}
}

fn game() -> Result<(), Box<dyn Error>> {
	let answer = words::get_random_answer()?;
	let allowed_guesses = words::allowed_guesses()?;

	let mut tries: u8 = 6;

	while tries > 0 {
		let mut guess = String::new();
		println!("\nGuess the word. Tries: {}", tries);

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line.");

		let guess = guess.trim().to_lowercase();

		if !allowed_guesses.contains_key(&guess) {
			println!("Invalid guess.");
			continue;
		}

		if guess == answer {
			println!("Congratulations, you got it!");
			break;
		}

		let hint = get_hint(&guess, &answer);

		println!("{}", hint);
		tries -= 1;
	}

	if tries == 0 {
		println!("You ran out of tries. The answer was ''{}'.", answer);
	}

	Ok(())
}

pub fn get_hint(guess: &str, answer: &str) -> String {
	let mut output: [char; 5] = ['⬛'; 5];

	let mut contains_map: HashMap<char, u8> = HashMap::new();

	let guess_chars: Vec<char> = guess.as_bytes().iter().map(|x| *x as char).collect();
	let answer_chars: Vec<char> = answer.as_bytes().iter().map(|x| *x as char).collect();

	// green pass
	for n in 0..5 {
		match guess_chars[n] == answer_chars[n] {
			true => output[n] = '🟩',
			false => {
				let count = contains_map.entry(answer_chars[n]).or_insert(0);
				*count += 1;
			}
		}
	}

	// yellow pass
	for n in 0..5 {
		if guess_chars[n] == answer_chars[n] {
			continue;
		}
		let count = contains_map.entry(guess_chars[n]).or_default();
		if *count == 0 {
			continue;
		}
		output[n] = '🟨';
		*count -= 1;
	}

	output.iter().collect::<String>()
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn assert_get_hint_handles_duplicates_correctly() {
		// handle duplicates correctly
		assert_eq!(get_hint("bbbzb", "xxxbb"), "🟨⬛⬛⬛🟩");
		assert_eq!(get_hint("zbbzb", "xxxbb"), "⬛🟨⬛⬛🟩");
		assert_eq!(get_hint("zzbzb", "xxxbb"), "⬛⬛🟨⬛🟩");
	}
}
