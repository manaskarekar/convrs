
pub fn convert(source_file: &String, source_format: &String, dest_format: &String) -> () {
	println!("Converting file {:#?} from {} to {} ...", source_file, source_format, dest_format);
	match source_format.as_str() { //handle lowercase
		".vim" | "vim" | "vi" | "gvim" => println!("Source: vim/vi/gvim"),
		".jedit" | "jedit" | "jEdit" => println!("Source: jEdit"),
		&_ => println!("default"),
	}
}