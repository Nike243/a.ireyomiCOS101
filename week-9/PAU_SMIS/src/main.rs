use std::fs::File;
use std::io::Write;

fn main() {
	println!("__________________PAU - SMIS ____________________________________");

	let name = vec! ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
	let matric_no = vec! ["ACC10211111","ECO10110101","CSC10328828","EE11020202","MEE10202001"];
	let department = vec! ["Accounting","Economics","Computer","Electrical","Mechanical"];
	let level = vec! ["300","100","200","200","100"];

	let mut file = std::fs::File::create("student_records.csv").expect("could not find file");


	for i in 0..name.len() {
		println!("Name: {}", name[i]);
		println!("matric no: {}",matric_no[i]);
		println!("department: {}",department[i]);
		println!("level: {}",level[i]);
		println!("_________________________________________________________________");


		writeln!(file, "Name: {}",name[i]);
		writeln!(file, "matric no: {}",matric_no[i]);
		writeln!(file, "department: {}",department[i]);
		writeln!(file, "level: {}",level[i]);
	}


	println!("student record saved successfully!");


}
