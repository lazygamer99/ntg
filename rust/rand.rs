use rand::Rng;
fn main(){
	let secret_number = rand::thread_rng().gen_range(1..=10);
	println!("Guess a number between 1 to 10");
	
	loop{
		let mut guess = String::new();
		std::io::stdin().read_line(&mut guess).expect("Failed to read line");
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		if guess == secret_number{
			println!("You guessed it");
			break;
		} else {
			println!("Wrong! Try again.");
		}
	}
}
