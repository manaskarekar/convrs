use ir::{read_file, tokenize, IntermediateRepr};
//use regex::Regex;
use std::fs::File;
use plist::Plist;

/* Todo for the reader
	parse the {} as objects
	map them to objects/dicts

*/
pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading Sublime Text 3 color scheme..");

	//let scheme_data = read_file(&infile);
	//let ir:  IntermediateRepr = Default::default();
	let ir = IntermediateRepr { name: "default".to_string(), ..Default::default() };
	let mut f = File::open(&infile).expect("Failed to read file.");

	//let re = Regex::new(r"\{(?s).*\}").unwrap();
	//let cap = re.captures(scheme_data.as_str()).unwrap();
	//println!("{:#?}", cap);

	let pl = Plist::read(f);
	println!("{:#?}", pl.unwrap());



	/*IntermediateRepr{
		name : "#000000".to_string(),

		//fgcolor : lines[12].split(' ').collect::<Vec<&str>>()[6].to_string(),
		//bgcolor : lines[31].split(' ').collect::<Vec<&str>>()[6].to_string(),
		fgcolor : "#000000".to_string(),
		bgcolor : "#FFFFFF".to_string(),

		keyword1 : "#000000".to_string(),
		keyword2 : "#000000".to_string(), //TODO: Temporarily using keyword1 val
		keyword3 : "#000000".to_string(),
		keyword4 : "#000000".to_string(),

		comment1 : "#000000".to_string(),
		digit 	 : "#000000".to_string(),
		operator : "#000000".to_string(),
		function : "#000000".to_string(),

		literal1 : "#000000".to_string(),
		literal2 : "#000000".to_string(),
		literal3 : "#000000".to_string(),
		caretColor : "#000000".to_string(),
		selectionColor : "#000000".to_string(),
		eolMarkerColor : "#000000".to_string(),
		lineHighlightColor : "#000000".to_string(),

	}*/

	 ir
}

pub fn write_scheme() {}