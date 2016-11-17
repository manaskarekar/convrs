use rustc_serialize::{json};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct IRAttribute {
	color: String,
	bgcolor: String,
	style: String, //example: "biu" -> bold and italic and underscored if supported.
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct IntermediateRepr {
	keyword1 : String, //TODO: Keeping it simple for now, change these to IRAttribute.
	keyword2 : String,
	keyword3 : String,
	keyword4 : String,

	currentLineColor : String,

	foldLine0 : String,
	foldLine1 : String,
	foldLine2 : String,
	foldLine3 : String,
}

pub fn ir_to_json(ir: &IntermediateRepr) -> String {
	let encoded = json::encode(&ir).unwrap();
	println!("\nEncoded: {:#?}", &encoded);

	encoded
}

pub fn json_to_ir(json_data: &String) -> IntermediateRepr {
	let decoded: IntermediateRepr = json::decode(json_data).unwrap();
	println!("\njson_data: {:#?}", &json_data);

	decoded
}

pub fn write_json(ir_file: String) {
	//json file
}

pub fn read_json(ir_file: String) -> String {
	println!("json file: {}", &ir_file);

	let mut f = File::open(&ir_file).expect("Failed to read .ir file.");
	let mut data = String::new();

	f.read_to_string(&mut data).unwrap();
	//let json_data = json::Json::from_str(&data).unwrap(); //returns json::Json object

	data
}