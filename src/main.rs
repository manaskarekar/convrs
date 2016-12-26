//Convrs: A tool to convert color schemes between editors.

extern crate plist;
//extern crate regex;
//extern crate rustc_serialize;

mod profiles;
mod ir;
mod convert;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if (args.len() != 4) {
		println!("Invalid number of args: Use form: convrs <filename> <source_format> <destination_format>.")
	} else {
		convert::convert(&args[1], &args[2], &args[3]);
	}
	//profiles::vim::tokenize("src/test.json".to_string());
}

/*
use ir::*;

{
	// Misc code snippets until I grok Rust syntax
	for x in env::args(){
		println!("{}", x);
	}

	let data = ir::read_file("src/test.json".to_string());
	//println!("{:#?}", &data);
	let ir_obj = ir::json_to_ir(&data);
	let json_string = ir_to_json(&ir_obj);
}
*/