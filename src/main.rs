use std::env;
use std::io::prelude::*;
use std::fs::File;

fn uwufy(input : &str) -> String{
	let mut temp : String = input.replace("na", "nya");
	temp = temp.replace("th", "d");
	temp = temp.replace("l", "w");
	temp = temp.replace("r", "w");
	temp = temp.replace("ohh", "uh");

	temp = temp.replace("ou", "uw");

	temp = temp.replace("Th", "D");
	temp = temp.replace("tH", "D");

	temp = temp.replace("TH", "D");
	temp = temp.replace("L", "W");
	temp = temp.replace("R", "W");
	temp = temp.replace("Ohh", "Uh");
	return temp;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		println!("Not enough arguments, try -h.");
		return;
	}
	if args[1] == "-h" {
		println!("Usage uwu_rust \"hello world\"");
		return;
	};
	if args[1] == "-f" {
		if args.len() <= 2 {
			println!("Not enough arguments for -f, try -h.");
			return;
		}
		let mut file = File::open(&args[2]).expect("Failed to open file");
		let mut contents = String::new();
		file.read_to_string(&mut contents).expect("Failed to read from file");
		println!("{}", uwufy(&contents));
	}
	else{
		println!("{}", uwufy(&args[1]));
	}
}
