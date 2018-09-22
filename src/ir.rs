#![allow(non_snake_case)]

use std::fs::File;
use std::io::{Read, Write};
//use std::path::Path;
//use std::error::Error;

/*
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct IRAttribute {
	color: String,
	bgcolor: String,
	style: String, //example: "biu" -> bold and italic and underscored if supported.
}
*/

//#[derive(RustcDecodable, RustcEncodable, Debug, Default)]
#[derive(Debug, Default)]
pub struct IntermediateRepr {

	pub name : String,
	pub fgcolor : String,
	pub bgcolor : String,
	pub keyword1 : String,
	pub keyword2 : String,
	pub keyword3 : String,
	pub keyword4 : String,
	pub comment1 : String,
	pub digit : String,
	pub operator : String,
	pub function : String,
	pub literal1 : String,
	pub literal2 : String,
	pub literal3 : String,
	pub caretColor : String,
	pub selectionColor : String,
	pub eolMarkerColor : String,
	pub lineHighlightColor : String,

}

/*
pub fn tokenize(infile: &String) {
	let raw_data = read_file(infile);
	println!("raw_data:{:#?}", &raw_data);
}
*/

pub fn read_file(filename: &str) -> String {
	println!("Reading file: {}", &filename);

	let mut f = File::open(&filename).expect("Failed to read file.");
	let mut data = String::new();

	f.read_to_string(&mut data).unwrap();
	//let json_data = json::Json::from_str(&data).unwrap(); //returns json::Json object

	data
}

pub fn write_file(filename: &str, data: &str) {
	println!("Writing file: {}", &filename);

	let mut f = File::create(filename).expect("Failed to create output file.");
	//let mut f = try!(File::create("foo.txt"));

	f.write_all(data.as_bytes()).expect("Failed to write to output file.")
}
