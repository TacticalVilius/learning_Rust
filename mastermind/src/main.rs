#![feature(core)]
#![feature(io)]
#![feature(rand)]
#![feature(collections)]

use std::rand;
use std::old_io;
use std::collections::HashSet;
use std::cmp::min;
use std::iter::repeat;

fn main() {
	let all_colours = ['W', 'Y', 'O', 'R', 'G', 'B', 'Z', 'P', 'U', 'A'];

	println!("Welcome to MASTERMIND");
	
	let colour_num = request_colour_num(all_colours.len() as u32) as usize;
	let mut used_colours: Vec<char> = repeat(' ').take(colour_num).collect();
	pick_random_colours(&all_colours, used_colours.as_mut_slice());
	
	let turns_num = request_turns_num();
	
	print!("Available colours: ");
	for colour in used_colours.iter() { print!("{} ", colour); }
	print!("\n\n");
	
	let answer: Vec<char> = range(0, colour_num).map(|_| get_random_colour_letter(used_colours.as_slice())).collect();
	let mut sorted_answer = answer.clone();
	sorted_answer.sort();
	
	let mut board = String::new();
	let mut counter = 1;
	
	loop {
		print!("\n\nGUESS:\t\t");
		
		let input = old_io::stdin()
					.read_line()
					.ok()
					.expect("Failed to read player input");
		
		let guess = match validate_player_input(input.trim(), answer.len(), used_colours.as_slice()) {
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
		
		if correct_colour_place == answer.len() as u32 {
			print!("\nCORRECT!\n");
			return;
		}
		
		let mut correct_colour = guess_answer_cmp_colour(&guess_vec, &answer);
		if correct_colour > correct_colour_place { correct_colour = correct_colour - correct_colour_place; }
		else { correct_colour = 0; }
		
		board = board + &(counter.to_string()) + "...\t\t" + guess + "\t" + &(correct_colour_place.to_string()) + "/" + &(correct_colour.to_string()) + "\n";
		print!("\n{}", board);
		counter = counter + 1;
		if counter > turns_num {
			print!("\nYou lost!\n");
			return;
		}
	}
}

fn pick_random_colours(all_colours: &[char], picked_colours: &mut [char]) {
	let mut picked_indices = HashSet::new();
	for i in 0..picked_colours.len() {
		let next_colour_index = rand::random::<usize>() % (all_colours.len() - i);
		let mut current_index: usize = 0;
		for j in 0..all_colours.len() {
			if !picked_indices.contains(&j) {
				if current_index < next_colour_index { current_index = current_index + 1; }
				else if current_index == next_colour_index {
					picked_colours[i] = all_colours[j];
					picked_indices.insert(j);
					break;
				}
			}
		}
	}
}

fn request_colour_num(max: u32) -> u32 {
	loop {
		println!("Choose amount of colours (max. {}): ", max);
		let input = old_io::stdin().read_line().ok().expect("Failed to read line");
		let input_num: u32 = match input.trim().parse() {
			Some(num) => num,
			None => {
				println!("Please input a number");
				continue;
			}
		};
		if input_num > max || input_num < 1 {
			println!("Number on of colours must be between 1 and {}", max);
			continue;
		}
		else {
			return input_num;
		}
	}
}

fn request_turns_num() -> u32 {
	loop {
		println!("Choose amount of turns: ");
		let input = old_io::stdin().read_line().ok().expect("Failed to read line");
		let input_num: u32 = match input.trim().parse() {
			Some(num) => num,
			None => {
				println!("Please input a number");
				continue;
			}
		};
		if input_num < 1 {
			println!("Number of turns must be 1 or more");
			continue;
		}
		else {
			return input_num;
		}
	}
}

fn get_random_colour_letter(used_colours: &[char]) -> char {
	used_colours[rand::random::<usize>() % used_colours.len()]
}

fn validate_player_input<'a>(input: &'a str, correct_length: usize, used_colours: &[char]) -> Result<&'a str, String> {
	if input.len() != correct_length { Err("Length of guess must be ".to_string() + &(correct_length.to_string())) }
	else {
		for c in input.chars() {
			if !valid_colour_letter(c, used_colours) { return Err(format!("Invalid colour letter {}", c)); }
		}
		Ok(input)
	}
}

fn valid_colour_letter(letter: char, all_colours: &[char]) -> bool {
	match all_colours.iter().find(|&c| *c == letter) {
		Some(_) => true,
		None => false
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
	let mut checked = HashSet::new();
	
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