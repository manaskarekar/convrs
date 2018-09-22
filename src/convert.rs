use profiles::*;
use ir::*;

pub fn convert(source_file: &str, source_format: &str, dest_format: &str) {
	println!("Converting file {:#?}: {} -> {}", source_file, source_format, dest_format);

	let mut source_data: IntermediateRepr = Default::default();

	let formats_available = [".vim" , "vim" , "vi" , "gvim" , ".gvim", ".jedit" , "jedit", "paletton_txt", "tmtheme", "tm"]; //make dict and use that below

	match source_format.to_lowercase().as_str() {
		".vim" | "vim" | "vi" | "gvim" | ".gvim" => source_data = vim::read_scheme(source_file),
		".jedit" | "jedit" => source_data = jedit::read_scheme(source_file),
		"pt" | "paletton" | "paletton_txt" => source_data = paletton_txt::read_scheme(source_file),
		"sublime-theme" | "subl" | "sublimetext" => source_data = st3::read_scheme(source_file),
		"tmtheme" | "tm" => source_data = tm::read_scheme(source_file),
		&_ => println!("Error: Unknown source filetype. Choose one from: {:?}", &formats_available),		//TODO: How to handle this "None"?
	};

	match dest_format.to_lowercase().as_str() {
		".vim" | "vim" | "vi" | "gvim" | ".gvim" => vim::write_scheme(),
		".jedit" | "jedit" => jedit::write_scheme(&source_data),
		"pt" | "paletton" | "paletton_txt" => paletton_txt::write_scheme(),
		"sublime-theme" | "subl" | "sublimetext" => st3::write_scheme(),
		"tmtheme" | "tm" => tm::write_scheme(),
		&_ => println!("Error: Unknown dest filetype. Choose one from: {:?}", &formats_available),
	}
}
