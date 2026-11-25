use std::io;

fn main() {
    let levels = vec![
        ("APS 1-2", "Intern", "-", "Paralegal", "Placement"),
        ("APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"),
        ("EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", "CEO", "Dean", "Partner", "Principal"),
    ];

    println!("================================");
    println!("      APS LEVEL CHECKER");
    println!("================================");

    println!("Choose your profession:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice: usize = match choice.trim().parse() {
        Ok(n) if n >= 1 && n <= 4 => n,
        _ => {
            println!("Invalid profession choice!");
            return;
        }
    };

    println!("Enter your years of experience:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let experience: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let row = match experience {
        0..=2 => 0,
        3..=4 => 1,
        5..=8 => 2,
        9..=10 => 3,
        11..=13 => 4,
        _ => 5,
    };

    let profession = match choice {
        1 => "Office Admin",
        2 => "Academic",
        3 => "Lawyer",
        4 => "Teacher",
        _ => "Unknown",
    };

    let position = match choice {
        1 => levels[row].1,
        2 => levels[row].2,
        3 => levels[row].3,
        4 => levels[row].4,
        _ => "-",
    };

    println!("\n================================");
    println!("          YOUR RESULT");
    println!("================================");
    println!("Profession: {}", profession);
    println!("Experience: {} years", experience);
    println!("APS Level: {}", levels[row].0);
    println!("Position: {}", position);
    println!("================================");
}