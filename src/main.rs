use std::fs;
use std::env;

fn print_file_content(fname: &str) {
	let data = fs::read_to_string(fname);
	match data {
		Ok(v) => print!("{}", v),
		Err(e) => eprintln!("{}: {}", fname, e),
	}
}

fn print_stdin() {

}

fn usage() {
	let msg = r"";
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
