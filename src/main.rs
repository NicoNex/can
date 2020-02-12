use std::fs;
use std::env;
use std::io::{self, Read};

fn print_file_content(fname: &str) {
	let data = fs::read_to_string(fname);
	match data {
		Ok(v) => print!("{}", v),
		Err(e) => eprintln!("{}: {}", fname, e),
	}
}

fn read_stdin() -> Result<String, io::Error> {
	let mut buf = String::new();
	let stdin = io::stdin();
	let mut handle = stdin.lock();
	handle.read_to_string(&mut buf)?;
	Ok(buf)
}

fn print_stdin() {
	match read_stdin() {
		Ok(s) => print!("{}", s),
		Err(e) => eprintln!("{}", e),
	}
}

fn usage() {
	let msg = r"can - Mitt inzim i file e stamba all'stdout.";
	println!("{}", msg);
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		for i in 1..args.len() {
			print_file_content(&args[i]);
		}
	} else {
		print_stdin();
	}
}
