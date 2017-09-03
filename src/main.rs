//Convrs: A tool to convert color schemes between editors.

extern crate plist;
//extern crate regex;

mod profiles;
mod ir;
mod convert;

use std::env;
use std::path::Path;


fn main() {
	let args: Vec<String> = env::args().collect(); //env::args returns an iterator, collect gives the vector
	if args.len() != 4 {
		println!("Invalid number of args: Use form: convrs <filename> <source_format> <destination_format>.")
	} else {
		let src_path = Path::new(&args[1]);

		if src_path.is_dir() {
			for f in src_path.read_dir().expect("Failed to read source directory.") {
				if let Ok(f) = f {
					println!("{:?}", f);
					if f.path().is_file() {
						convert::convert(&f.path().into_os_string().into_string().unwrap(), &args[2], &args[3]);
					}
				}				
			}
		} else {  
			convert::convert(&args[1], &args[2], &args[3]);
		}
	}
}
