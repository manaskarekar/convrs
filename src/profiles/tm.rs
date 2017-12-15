use ir::{IntermediateRepr};

use std::fs::File;
use plist::Plist;

//use std::io::BufReader;

pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading Textmate color scheme (plist)..");

	//let scheme_data = read_file(&infile);
	//let ir:  IntermediateRepr = Default::default();


	let f = File::open(&infile).expect("Failed to read file.");
	let pl = Plist::read(f);
	//println!("{:#?}", pl.unwrap().as_dictionary().unwrap().get("uuid"));
	println!("{:#?}", pl.unwrap().as_dictionary().unwrap().get("Lines").unwrap().as_array().unwrap().get(0).unwrap());

	//let mut reader = BufReader::new(f);
	//let pl = Plist::from_reader(&mut reader).unwrap();
	//println!("{:#?}", pl);

	let ir = IntermediateRepr { name: "default".to_string(), ..Default::default() };
	//IntermediateRepr{
	//	name : "#000000".to_string(),
    //
	//	//fgcolor : lines[12].split(' ').collect::<Vec<&str>>()[6].to_string(),
	//	//bgcolor : lines[31].split(' ').collect::<Vec<&str>>()[6].to_string(),
	//	fgcolor : "#000000".to_string(),
	//	bgcolor : "#FFFFFF".to_string(),
    //
	//	keyword1 : "#000000".to_string(),
	//	keyword2 : "#000000".to_string(), //TODO: Temporarily using keyword1 val
	//	keyword3 : "#000000".to_string(),
	//	keyword4 : "#000000".to_string(),
    //
	//	comment1 : "#000000".to_string(),
	//	digit 	 : "#000000".to_string(),
	//	operator : "#000000".to_string(),
	//	function : "#000000".to_string(),
    //
	//	literal1 : "#000000".to_string(),
	//	literal2 : "#000000".to_string(),
	//	literal3 : "#000000".to_string(),
	//	caretColor : "#000000".to_string(),
	//	selectionColor : "#000000".to_string(),
	//	eolMarkerColor : "#000000".to_string(),
	//	lineHighlightColor : "#000000".to_string(),
    //
	//}

	ir
}

pub fn write_scheme() {}
