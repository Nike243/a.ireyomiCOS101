
use std::fs;
use std::io;

fn display_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("\n{}", contents);
        }
        Err(error) => {
            println!("Error opening file: {}", error);
        }
    }
}

fn administrator() {
    println!("ADMINISTRATOR");
    display_file("globacom_db.sql");
}

fn project_manager() {
    println!("PROJECT MANAGER");
    display_file("project_tb.sql");
}

fn employee() {
    println!("EMPLOYEE");
    display_file("staff_tb.sql");
}

fn customer() {
    println!("CUSTOMER");
    display_file("customer_tb.sql");
}

fn vendor() {
    println!("VENDOR");
    display_file("dataplan_tb.sql");
}

fn main() {
    println!("WELCOME TO GLOBACOM DATABASE SYSTEM");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    println!("\nSelect your role:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => administrator(),
        "2" => project_manager(),
        "3" => employee(),
        "4" => customer(),
        "5" => vendor(),
        _ => println!("Invalid selection"),
    }
}

