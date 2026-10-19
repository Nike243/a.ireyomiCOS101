use std::fs::File;
use std::io::Write;

fn main() {
    // Create the file
    let mut file = File::create("nigerian_breweries.txt")
        .expect("Could not create file");

    // Write the title
    writeln!(file, "NIGERIAN BREWERIES").unwrap();
    writeln!(file, "==================").unwrap();

    // Lager category
    writeln!(file, "\nLAGER").unwrap();
    writeln!(file, "33 Export").unwrap();
    writeln!(file, "Desperados").unwrap();
    writeln!(file, "Goldberg").unwrap();
    writeln!(file, "Gulder").unwrap();
    writeln!(file, "Heineken").unwrap();
    writeln!(file, "Star").unwrap();

    // Stout category
    writeln!(file, "\nSTOUT").unwrap();
    writeln!(file, "Legend").unwrap();
    writeln!(file, "Turbo King").unwrap();
    writeln!(file, "Williams").unwrap();

    // Non-Alcoholic category
    writeln!(file, "\nNON-ALCOHOLIC").unwrap();
    writeln!(file, "Maltina").unwrap();
    writeln!(file, "Amstel Malta").unwrap();
    writeln!(file, "Malta Gold").unwrap();
    writeln!(file, "Fayrouz").unwrap();

    // Display a success message
    println!("Nigerian Breweries file created successfully!");
}
