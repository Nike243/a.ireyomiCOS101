use std::io;

fn main() {
	println!("    RESTAURANT MENU   ");

	println!("P - Poundo Yam / Edikaiko Soup - ₦3,200");

	println!("F - Frie Rice & Chicken - ₦3,000");

	println!("A - Amala & Ewedu Soup - ₦2,500");

	println!("E - Eba & Egusi Soup - ₦2,000");

	println!("W - White rice and Stew - ₦2,500");

	// Ask for food type
	println!("Please enter the food type(P,F,A,E,W):");
	let mut food_type = String:: new();
	io::stdin().read_line(&mut food_type).expect("Failed to read input");
	let food_type = food_type.trim().to_uppercase();

	// Ask for quantity
	println!("Enter quantity:");
	let mut quantity = String::new();
	io::stdin().read_line(&mut quantity).expect("Failed to read input");
	let quantity: i32 = quantity.trim().parse().expect("Please enter a number");

	// Find the price
	let price: f64;

	if food_type == "P"{
		price = 3200.0;
	}
	else if food_type =="F"{
		price = 3000.0;
	}
	else if food_type == "A"{
		price = 2500.0;
	}
	else if food_type == "E"{
		price = 2000.0;
	}
	else if food_type == "W"{
		price = 2500.0;
	}
	else{
		println!("Not available");
		return;
	}

	//calculate total
	let total = price * quantity as f64;

	println!("Total before discount: ₦ {:.2}", total);

	// Check for discount
	if total > 10000.0 {
		let discount = total * 0.05;
		let final_total = total - discount;

		println!("Discount (5%) : ₦{:.2}", discount);
		println!("Final total: ₦{:.2}",final_total);
	}
	else {
		println!("No discount.");
		println!("Final total: ₦{:.2}", total);
	}

}