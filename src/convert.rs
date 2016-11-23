use profiles::*;
use ir::*;

pub fn convert(source_file: &String, source_format: &String, dest_format: &String) -> () {
	println!("Converting file {:#?} from {} to {} ...", source_file, source_format, dest_format);

	let mut source_data;

	match source_format.as_str() { //TODO: handle lowercase
		".vim" | "vim" | "vi" | "gvim" | ".gvim" => source_data = vim::read_scheme(source_file),
		".jedit" | "jedit" | "jEdit" => println!("Source: jEdit"),
		&_ => println!("Unknown source filetype."),
	};

	//match dest_format.as_str() {
	//	"" | "" | "" =>
    //
	//}
	()
}