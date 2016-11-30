use ir::{read_file, write_file, tokenize, IntermediateRepr};

pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading jedit scheme..");
	IntermediateRepr{
		name : "temp".to_string(),
		view_fgcolor : "temp".to_string(),
	}
}

pub fn write_scheme(scheme: &IntermediateRepr) {
	//TODO: use templating
	//TODO: handle edge cases
	//TODO: Rust rawstrings?

	let mut data: String;
	data = "#jEdit Editor Scheme\n#:mode=properties:lineSeparator=\\".to_string();
	data.push_str("n");
	//data = format!("{}", "");

	println!("{}", data);
	write_file(&"converted.jedit-scheme".to_string(), &data) //TODO: as_str is unstable
}