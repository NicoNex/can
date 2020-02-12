extern crate clap;

use std::fs;
use std::io::{self, Read};
use clap::{Arg, App};

fn print_file_content(fname: &str) {
	let data = fs::read_to_string(fname);
	match data {
		Ok(v) => print!("{}", v),
		Err(_) => eprintln!("Capo, ma c file v'liv? {} nan c ste!", fname),
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
				.help("Disc l file ch'a da stambà.")
				.required(true)
				.multiple(true))
			.get_matches();

	let fnames: Vec<_> = matches.values_of("FILE").unwrap().collect();
	let len = fnames.len();

	for i in 0..len {
		if fnames[i] == "-" {
			print_stdin();
		} else {
			print_file_content(&fnames[i]);
		}
	}
}
