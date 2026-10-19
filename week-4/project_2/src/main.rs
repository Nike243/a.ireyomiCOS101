use std::io;

fn main() {
	println!("Enter age:");
	let mut input = String::new();
	io::stdin ().read_line(&mut input).expect("Your input for your age could not be stored.");
	let age: u8 = input.trim().parse().expect("Please input a numeric number as your age.");
	input.clear();

	// es is experience status

    println!("Enter your es:");
	let mut es = String::new();
	io::stdin().read_line(& mut es).expect("Your input could not be stored.");
	let es = es.trim();
	println!("You entered: {}",es);
	input.clear();

	if es == "yes" && age >= 40
	{
		println!("₦1_560_000");
	}
	else if es == "yes" && age >=30 && age <=39
	{
		println!("₦1_480_000");
	}
	else if es == "yes" && age <=29
	{
		println!("₦1_300_000");
	}
	else if es == "none" 
	{
		println!("₦100_000");
	}

}