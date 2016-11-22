use ir::{read_file, IntermediateRepr};

pub fn tokenize(infile: String) {
	let raw_data = read_file(infile);
	println!("raw_data:{:#?}", &raw_data);
}


fn read_vimscheme() {} /*-> IntermediateRepr {


}

//write_vimscheme() {}*/