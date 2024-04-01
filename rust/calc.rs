use std::io;

fn main(){
	println!("=-=-=-=-=-=-=-=-=-=-CALCULATOR=-=-=-=-=-=-=-=-=-=-");
	
	println!("Enter the first number: ");
	let mut num1 = String::new();
	io::stdin().read_line(&mut num1).expect("Failed to read line.");
	
	println!("Enter the second number: ");
	let mut num2 = String::new();
	io::stdin().read_line(&mut num2).expect("Failed to read line.");
	
	let num1:u32 = match num1.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};
	
	let num2:u32 = match num2.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};
	
	loop {
		println!("Choose one of the following operations using the corresponding digits:
		1. Addition
		2. Subtraction
		3. Multiplication
		4. Division
		");
	
		let mut choice = String::new();
		io::stdin().read_line(&mut choice).expect("Failed to read line.");
	
		let choice:u32 = match choice.trim().parse() {
			Ok(num) => num,
			Err(_) => 0,
		};
	
		let quo = num1 as f32 / num2 as f32;
	
		if choice == 1 {
			println!("Sum: {}", num1 + num2);
			break;
		}
		else if choice == 2 {
			println!("Difference: {}", num1 - num2);
			break;
		}	
		else if choice == 3 {
			println!("Procuct: {}", num1 * num2);
			break;
		}
		else if choice == 4 {
			println!("Quotient:{}", quo);
			break;
		}
		else if choice > 4 || choice < 1{
			println!("Invalid choice. Choose a valid option: ");
			continue;
		}
	}
}
