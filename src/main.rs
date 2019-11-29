use std::env;

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

	println!("{}", uwufy(&args[1]));
}
