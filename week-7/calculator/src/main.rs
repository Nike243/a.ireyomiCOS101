use std::io;

fn main() {
    loop {
        println!("\n==============================");
        println!("      SHAPE CALCULATOR");
        println!("==============================");
        println!("1. Area of a trapezium");
        println!("2. Area of a rhombus");
        println!("3. Area of a parallelogram");
        println!("4. Surface area of a cube");
        println!("5. Volume of a cylinder");
        println!("6. Exit");
        println!("==============================");

        println!("Enter your numerical choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match choice {
            1 => println!("Area = {}", trapezium()),
            2 => println!("Area = {}", rhombus()),
            3 => println!("Area = {}", parallelogram()),
            4 => println!("Surface area = {}", cube()),
            5 => println!("Volume = {}", cylinder()),
            6 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn trapezium() -> f64 {
    let mut input = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter first base:");
    io::stdin().read_line(&mut input).unwrap();
    let base1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter second base:");
    io::stdin().read_line(&mut input).unwrap();
    let base2: f64 = input.trim().parse().unwrap();

    height / 2.0 * (base1 + base2)
}

fn rhombus() -> f64 {
    let mut input = String::new();

    println!("Enter first diagonal:");
    io::stdin().read_line(&mut input).unwrap();
    let d1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter second diagonal:");
    io::stdin().read_line(&mut input).unwrap();
    let d2: f64 = input.trim().parse().unwrap();

    d1 * d2 / 2.0
}

fn parallelogram() -> f64 {
    let mut input = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input).unwrap();
    let base: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input).unwrap();
    let altitude: f64 = input.trim().parse().unwrap();

    base * altitude
}

fn cube() -> f64 {
    let mut input = String::new();

    println!("Enter side:");
    io::stdin().read_line(&mut input).unwrap();
    let side: f64 = input.trim().parse().unwrap();

    6.0 * side * side
}

fn cylinder() -> f64 {
    let mut input = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input).unwrap();
    let radius: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    3.142 * radius * radius * height
}