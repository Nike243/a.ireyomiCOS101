use std::io;

fn main() {
	println!("Enter a:");
	 let mut input = String::new();
	 io::stdin().read_line(&mut input).unwrap();
	 let a:f64 = input.trim().parse().unwrap();
	 input.clear();

	 println!("Enter b:");  
	 let mut input = String::new();
	 io::stdin().read_line(&mut input).unwrap();
	 let b:f64 = input.trim().parse().unwrap();
	 input.clear();

	 println!("Enter c:");
	 let mut input = String::new();
	 io::stdin().read_line(&mut input).unwrap();
	 let c:f64 = input.trim().parse().unwrap();
	 input.clear();

	 let d =b*b - 4.0*a*c;
	 if d > 0.0 
	 {
	 let x1 = (-b + d.sqrt())/(2.0*a);
	 let x2 = (-b - d.sqrt())/(2.0*a);
	 println!("Two distinct root:");
	 println!("x1 = {}",x1);
	 println!("x2 = {}",x2);
	}  
	else if d == 0.0{
		let x = -b /(2.0*a);
		println!("Exactly one real root:");
		println!("x = {}", x);
	}
	else if d < 0.0 {
		println!("No real root.");
	}

}