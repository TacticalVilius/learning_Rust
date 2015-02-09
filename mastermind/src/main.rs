#![feature(io)]
#![feature(rand)]
#![feature(collections)]

use std::rand;
use std::old_io;
use std::collections::HashSet;
use std::cmp::min;

fn main() {
    println!("Welcome to MASTERMIND");
	println!("Available colours: G B Y R");
	println!("");
	
	let mut answer: [char; 4] = ['a'; 4];
	for element in answer.iter_mut() {
		*element = match get_random_colour_letter() {
			Some(x) => x,
			None => return
		}
	}
	let mut sorted_answer = answer.clone();
	sorted_answer.sort();
	
	let mut board = Box::new(String::new());
	let mut counter: i32 = 1;
	//*board = counter.to_string() + "...\t\t";
	
	loop {
		print!("\n{}\n\nGUESS:\t\t", *board);
		
		let input = old_io::stdin()
					.read_line()
					.ok()
					.expect("Failed to read player input");
		
		let guess = match validate_player_input(input.trim()) {
			Err(error_string) => {
				println!("{}", error_string);
				continue;
			},
			Ok(guess) => {
				guess
			}
		};
		
		let guess_vec = guess.chars().collect::<Vec<char>>();
		let correct_colour_place = guess_answer_cmp_colour_place(&guess_vec, &answer);
		
		if correct_colour_place == 4 {
			println!("CORRECT!");
			return;
		}
		
		let mut correct_colour = guess_answer_cmp_colour(&guess_vec, &answer);
		if correct_colour > correct_colour_place { correct_colour = correct_colour - correct_colour_place; }
		else { correct_colour = 0; }
		
		*board = (&*board).to_string() + &(counter.to_string()) + "...\t\t" + guess + "\t" + &(correct_colour_place.to_string()) + "/" + &(correct_colour.to_string()) + "\n";
		counter = counter + 1;
		if counter > 10 {return;}
	};
}

fn get_random_colour_letter() -> Option<char> {
	unsigned_int_to_colour_letter((rand::random::<u32>() % 4) + 1)
}

fn unsigned_int_to_colour_letter(i: u32) -> Option<char> {
	match i {
		1 => Some('G'),
		2 => Some('B'),
		3 => Some('Y'),
		4 => Some('R'),
		_ => {
			println!("ERROR: Cannot convert unsigned integer {} to a colour", i);
			None
		}
	}
}

fn validate_player_input(input: &str) -> Result<&str, String> {
	if input.len() != 4 { Err("Length of guess must be 4".to_string()) }
	else {
		for l in input.graphemes(true) {
			if valid_colour_letter(l) == false { return Err(format!("Invalid colour letter {}", l)); }
		}
		Ok(input)
	}
}

fn valid_colour_letter(letter: &str) -> bool {
	match letter {
		"G" => true,
		"B" => true,
		"Y" => true,
		"R" => true,
		_ => false
	}
}

fn guess_answer_cmp_colour_place(guess: &[char], answer: &[char]) -> u32 {
	let mut correct = 0;
	
	for i in 0..guess.len() {
		if guess[i] == answer[i] { correct = correct + 1; }
	}
	
	correct
}

fn guess_answer_cmp_colour(guess: &[char], answer: &[char]) -> u32 {
	let mut correct:u32 = 0;
	let mut checked: HashSet<char> = HashSet::new();
	
	for guess_el in guess.iter() {
		if !checked.contains(guess_el) {
			let g_amount = guess.iter().filter(|&x| x == guess_el).fold(0, |sum, _| sum + 1);
			let a_amount = answer.iter().filter(|&x| x == guess_el).fold(0, |sum, _| sum + 1);
			correct = correct + min(g_amount, a_amount);
			checked.insert(*guess_el);
		}
	}
	correct
}