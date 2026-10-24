use std::fs::File;
use std::io::Write;

fn main() {
	let commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
	let ministry = vec!["Internal Affairs","Justice","Defence","Power & Steel","Petroleum"];
	let zone = vec!["South west","North East","South South","South West","South East"];

	let mut file = std::fs::File::create("merged_records.csv").expect("could not find file");

	writeln!(file,"commissioner,ministry,zone").expect("could not write ")

	  for i in 0..commissioner.len() {
        println!("{}, {}, {}",commissioner[i],ministry[i],zone[i]);

        writeln!(file,"{},{},{}",commissioner[i],ministry[i],zone[i]).expect("could not write it");
    }

    println!("Records merged and saved successfully!");
}

