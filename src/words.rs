use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn get_random_answer() -> String {
	let possible_answers = possible_answers();
	possible_answers[rand::thread_rng().gen_range(0..possible_answers.len())].clone()
}

pub fn allowed_guesses() -> HashMap<String, bool> {
	let mut possible_answers = possible_answers();
	let mut allowed_guesses = get_file("src/words/allowed.json");
	allowed_guesses.append(&mut possible_answers);

	allowed_guesses.iter().map(|x| (x.clone(), true)).collect()
}

fn possible_answers() -> Vec<String> {
	get_file("src/words/answers.json")
}

fn get_file(path: &str) -> Vec<String> {
	let mut file = match File::open(path) {
		Ok(f) => f,
		Err(why) => panic!("{}", why),
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Ok(_) => (),
		Err(why) => panic!("{}", why),
	};

	match serde_json::from_str(&s) {
		Ok(v) => v,
		Err(why) => panic!("{}", why),
	}
}
