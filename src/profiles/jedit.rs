use ir::{read_file, write_file, tokenize, IntermediateRepr};

pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading jedit scheme..");
	let ir: IntermediateRepr = Default::default();
	ir
	//IntermediateRepr{
	//	name : "temp".to_string(),
	//	view_fgcolor : "temp".to_string(),
	//}
}

pub fn write_scheme(scheme: &IntermediateRepr) {
	//TODO: use templating
	//TODO: handle edge cases
	//TODO: Rust rawstrings?

	let mut data: String;
	data = "#jEdit Editor Scheme\n#:mode=properties:lineSeparator=\\".to_string();
	data.push_str("n\n");

	data.push_str(format!("scheme.name={}\n", &scheme.name).as_str());
	data.push_str(format!("view.fgcolor=\\{}\n", &scheme.view_fgcolor).as_str());
	data.push_str(format!("view.bgColor=\\{}\n", &scheme.view_bgcolor).as_str());
	data.push_str(format!("view.style.keyword1=\\{}\n", &scheme.text_keyword1).as_str());
	data.push_str(format!("view.style.keyword2=\\{}\n", &scheme.text_keyword2).as_str());
	data.push_str(format!("view.style.keyword3=\\{}\n", &scheme.text_keyword3).as_str());
	data.push_str(format!("view.style.keyword4=\\{}\n", &scheme.text_keyword4).as_str());

	println!("{}", data);
	write_file(&"converted.jedit-scheme".to_string(), &data)
}