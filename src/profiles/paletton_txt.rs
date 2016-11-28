use ir::{read_file, tokenize, IntermediateRepr};

pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading paletton_txt color palette..");

	let mut scheme = IntermediateRepr{}; //create new from scheme_data
	let scheme_data = read_file(&infile);
	let lines = scheme_data.lines();

	//TODO: Come back and do index-based (enumerate + find) to navigate the iterator.

	//if (lines_enum[i] == "*** Primary color:") {
	//
	//}



	scheme
}

pub fn write_scheme() {}