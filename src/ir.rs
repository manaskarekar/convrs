extern crate rustc_serialize;


use self::rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

pub struct IntermediateRepr {
	keyword1 : String,
	keyword2 : String,
	keyword3 : String,
	keyword4 : String,

	currentLineColor : String,

	foldLine0 : String,
	foldLine1 : String,
	foldLine2 : String,
	foldLine3 : String,
}

fn decode_from_ir(ir_file: String) {
	let mut f = File::open("template_ir.json").expect("Failed to read .ir file.");
	let mut data = String::new();

	f.read_to_string(&mut data).unwrap();

	let json_data = Json::from_str(&data).unwrap();
	//println!()
	//decode(json_data)

}

fn encode_to_ir(input_file: String) {}