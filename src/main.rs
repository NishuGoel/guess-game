use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	let secret_num = rand::thread_rng().gen_range(1, 101);
	loop {
		println!("Guess a number");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		println!("Guess is {}", guess);
		let guess: u32 = guess.trim().parse().expect("Please input a number only!");

		match guess.cmp(&secret_num) {
			Ordering::Less => println!("Too Small!"),
			Ordering::Greater => println!("Too Big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
