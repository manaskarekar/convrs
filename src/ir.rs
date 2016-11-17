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
	keyword1 : IRAttribute,
	keyword2 : IRAttribute,
	keyword3 : IRAttribute,
	keyword4 : IRAttribute,

	currentLineColor : IRAttribute,

	foldLine0 : IRAttribute,
	foldLine1 : IRAttribute,
	foldLine2 : IRAttribute,
	foldLine3 : IRAttribute,
}

pub fn ir_to_json(ir: String) {

}

pub fn json_to_ir(json_data: &String) -> IntermediateRepr {
	let decoded: IntermediateRepr = json::decode(json_data).unwrap();
	println!("\njson_data: {:#?}", &json_data);

	return decoded;
}

pub fn write_json(ir_file: String) {}

pub fn read_json2(ir_file: String) -> String {
	println!("json file: {}", &ir_file);

	let mut f = File::open(&ir_file).expect("Failed to read .ir file.");
	let mut data = String::new();

	f.read_to_string(&mut data).unwrap();

	data
}

pub fn read_json(ir_file: String) -> json::Json {
	println!("json file: {}", &ir_file);

	let mut f = File::open(&ir_file).expect("Failed to read .ir file.");
	let mut data = String::new();

	f.read_to_string(&mut data).unwrap();

	let json_data = json::Json::from_str(&data).unwrap();
	//println!("\ndata: {:#?}", json_data);

	json_data
}