//Convrs: A tool to convert color schemes between editors.

extern crate plist;
//extern crate regex;

mod profiles;
mod ir;
mod convert;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect(); //env::args returns an iterator, collect gives the vector
	if args.len() != 4 {
		println!("Invalid number of args: Use form: convrs <filename> <source_format> <destination_format>.")
	} else {
		convert::convert(&args[1], &args[2], &args[3]);
	}
}
