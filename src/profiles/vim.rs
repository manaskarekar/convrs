use ir::{read_file, tokenize, IntermediateRepr};

pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading vim scheme..");
	IntermediateRepr{
		name : "temp".to_string(),
		view_fgcolor : "temp".to_string(),
	}
}

pub fn write_scheme() {}