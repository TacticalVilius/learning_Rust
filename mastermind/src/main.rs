use std::rand;

fn main() {
    println!("Welcome to MASTERMIND");
	println!("Available colours: G B Y R");
	println!("");
	
	let mut answer = [""; 4];
	for i in 0u32..4 {
		answer[i] = "a";
	}
	
	print!("Answer is ");
	for element in answer.iter() {
		print!("{}", element);
	}
	println!("\n");
	
	let mut counter: i32 = 1;
	
	loop {
		print!("{}", counter);
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