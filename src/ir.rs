use rustc_serialize::{json};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct IRAttribute {
	color: String,
	bgcolor: String,
	style: String, //example: "biu" -> bold and italic and underscored if supported.
}


#[derive(RustcDecodable, RustcEncodable, Debug, Default)]
pub struct IntermediateRepr {
	pub text_keyword1 : String, //TODO: Keeping it simple for now, change these to IRAttribute.
	pub text_keyword2 : String,
	pub text_keyword3 : String,
	pub text_keyword4 : String,
    
	pub currentLineColor : String,
    
	//foldLine0 : String,
	//foldLine1 : String,
	//foldLine2 : String,
	//foldLine3 : String,

	pub name : String,
	pub view_fgcolor : String,
	pub view_bgcolor : String,

	//view_linehighlightcolor : String,
	//view_caretcolor : String,

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

pub fn tokenize(infile: &String) {
	let raw_data = read_file(infile);
	println!("raw_data:{:#?}", &raw_data);
}

pub fn read_file(filename: &String) -> String {
	println!("reading file: {}", &filename);

	let mut f = File::open(&filename).expect("Failed to read file.");
	let mut data = String::new();

	f.read_to_string(&mut data).unwrap();
	//let json_data = json::Json::from_str(&data).unwrap(); //returns json::Json object

	data
}

pub fn write_file(filename: &String, data: &String) {
	println!("writing file: {}", &filename);

	let mut f = File::create(filename).expect("Failed to create output file.");
	//let mut f = try!(File::create("foo.txt"));

	f.write_all(data.as_bytes()).expect("Failed to write to output file.")
}
