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

fn stdin_mode(){
	let mut buffer = String::new();
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	
	'mainloop: loop{
		let result = handle.read_line(&mut buffer);

		match result{
			Ok(res) => {
				if res == 0{
					break 'mainloop;
				}

				print!("{}", uwufy(&buffer));
			},
			Err(err) => {
				println!("Failed to read line {}", err);
			}
		}

	}


}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		stdin_mode();
		return;
	}
	if args[1] == "-h" {
		println!("You can use with files with the");
		println!("\t-f option fx. uwu_rust -f ./test.txt");
		println!("You can also use pipes with the \'|\' operator");
		println!("fx. echo \"The train is late.\" | uwu_rust");
		println!("And the last way to use it is just passing a string.");
		println!("fx. uwu_rust \"The train is late.\"");
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
