use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn get_random_answer() -> Result<String, Box<dyn Error>> {
	let possible_answers = possible_answers()?;
	Ok(possible_answers[rand::thread_rng().gen_range(0..possible_answers.len())].clone())
}

pub fn allowed_guesses() -> Result<HashMap<String, bool>, Box<dyn Error>> {
	let mut possible_answers = possible_answers()?;
	let mut allowed_guesses = get_file("src/words/allowed.json")?;

	allowed_guesses.append(&mut possible_answers);

	Ok(allowed_guesses.iter().map(|x| (x.clone(), true)).collect())
}

fn possible_answers() -> Result<Vec<String>, Box<dyn Error>> {
	get_file("src/words/answers.json")
}

fn get_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
	let mut file = File::open(path)?;

	let mut s = String::new();
	file.read_to_string(&mut s)?;

	match serde_json::from_str::<Vec<String>>(&s) {
		Ok(x) => Ok(x),
		Err(e) => Err(Box::new(e)),
	}
}
