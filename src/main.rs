mod words;
use std::collections::HashMap;
use std::io;

pub fn main() {
	let answer = words::get_random_answer();
	let allowed_guesses = words::allowed_guesses();

	loop {
		let mut guess = String::new();
		println!("\nGuess the word:");

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line.");

		let guess = guess.trim().to_lowercase();

		if !allowed_guesses.contains_key(&guess) {
			println!("Invalid guess.");
			continue;
		}

		let mut contains_map: HashMap<char, u8> = HashMap::new();

		let mut output: [char; 5] = ['⬛'; 5];

		let guess_chars: Vec<char> = guess.as_bytes().iter().map(|x| *x as char).collect();
		let answer_chars: Vec<char> = answer.as_bytes().iter().map(|x| *x as char).collect();

		// green pass
		for n in 0..5 {
			match guess_chars[n] == answer_chars[n] {
				true => output[n] = '🟩',
				false => {
					contains_map.entry(answer_chars[n]).or_insert(1);
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

		println!("{}", output.iter().collect::<String>());
	}
}
