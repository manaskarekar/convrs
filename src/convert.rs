use profiles::*;
use ir::*;

pub fn convert(source_file: &String, source_format: &String, dest_format: &String) -> () {
	println!("Converting file {:#?}: {} -> {}", source_file, source_format, dest_format);

	let mut source_data: IntermediateRepr = Default::default();

	match source_format.as_str() { //TODO: handle lowercase
		".vim" | "vim" | "vi" | "gvim" | ".gvim" => source_data = vim::read_scheme(source_file),
		".jedit" | "jedit" | "jEdit" => source_data = jedit::read_scheme(source_file),
		"paletton_txt" => source_data = paletton_txt::read_scheme(source_file),
		&_ => println!("Unknown source filetype."),		//TODO: How to handle this "None"?
	};

	match dest_format.as_str() {
		".vim" | "vim" | "vi" | "gvim" | ".gvim" => vim::write_scheme(),
		".jedit" | "jedit" | "jEdit" => jedit::write_scheme(&source_data),
		"paletton_txt" => paletton_txt::write_scheme(),
		&_ => println!("Unknown dest filetype."),
	}
	()
}