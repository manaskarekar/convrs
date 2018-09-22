use ir::{IntermediateRepr};

pub fn read_scheme(infile: &str) -> IntermediateRepr {
	println!("Reading vim scheme..{}", infile);
	let ir: IntermediateRepr = Default::default();
	ir
	//IntermediateRepr{
	//	name : "temp".to_string(),
	//	view_fgcolor : "temp".to_string(),
	//}
}

pub fn write_scheme() {}
