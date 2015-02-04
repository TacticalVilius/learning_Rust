use std::rand;
use std::old_io;

enum PlayerInput {
	ValidInput(String),
	ErrorReason(String)
}

fn main() {
    println!("Welcome to MASTERMIND");
	println!("Available colours: G B Y R");
	println!("");
	
	let mut answer: Vec<&str> = Vec::new();
	for i in 0..4 {
		answer.push(get_random_colour_letter());
	}
	
	print!("Answer is: ");
	for element in answer.iter() {
		print!("{}", element);
	}
	println!("");
	
	let mut counter: i32 = 1;
	
	loop {
		print!("{}...\t\t", counter);
		
		let input = old_io::stdin()
					.read_line()
					.ok()
					.expect("Failed to read player input");
		
		let guess = match validate_player_input(input.trim()) {
			PlayerInput::ErrorReason(error_string) => {
				println!("{}", error_string);
				continue;
			},
			PlayerInput::ValidInput(guess) => {
				guess
			}
		};
		
		
		
		counter = counter + 1;
		if counter > 10 {return;}
	};
}

fn get_random_colour_letter() -> &'static str {
	unsigned_int_to_colour_letter((rand::random::<u32>() % 4) + 1)
}

fn unsigned_int_to_colour_letter(i: u32) -> &'static str {
	match i {
		1 => "G",
		2 => "B",
		3 => "Y",
		4 => "R",
		_ => {
			println!("ERROR: Cannot convert unsigned integer {} to a colour", i);
			""
		}
	}
}

fn validate_player_input(input: &str) -> PlayerInput {
	if input.len() != 4 { PlayerInput::ErrorReason("Length of guess must be 4".to_string()) }
	else {
		for l in input.graphemes(true) {
			if valid_colour_letter(l) == false { return PlayerInput::ErrorReason(format!("Invalid colour letter {}", l)); }
		}
		PlayerInput::ValidInput(input.to_string())
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