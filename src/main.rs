extern crate clap;

use std::fs;
use std::io::{self, Read};
use clap::{Arg, App};

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

fn main() {
	let matches = App::new("can")
			.author("Nicolò Santamaria <nicolo.santamaria@gmail.com>")
			.about("Mitt inzim i file e stamba all'stdout.")
			.arg(Arg::with_name("FILE")
				.help("Sets the input files to use.")
				.required(true)
				.multiple(true))
			.get_matches();

	let fnames: Vec<_> = matches.values_of("FILE").unwrap().collect();
	let len = fnames.len();

	if len > 1 {
		for i in 0..len {
			if fnames[i] == "-" {
				print_stdin();
			} else {
				print_file_content(&fnames[i]);
			}
		}
	}
}
