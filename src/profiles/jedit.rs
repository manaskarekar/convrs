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
	data.push_str("n:\n");

	data.push_str(format!("scheme.name={}\n", &scheme.name).as_str());
	data.push_str(format!("view.fgcolor=\\{}\n", &scheme.fgcolor).as_str());
	data.push_str(format!("view.bgColor=\\{}\n", &scheme.bgcolor).as_str());
	data.push_str(format!("view.style.keyword1=color\\:\\{}\n", &scheme.keyword1).as_str());
	data.push_str(format!("view.style.keyword2=color\\:\\{}\n", &scheme.keyword2).as_str());
	data.push_str(format!("view.style.keyword3=color\\:\\{}\n", &scheme.keyword3).as_str());
	data.push_str(format!("view.style.keyword4=color\\:\\{}\n", &scheme.keyword4).as_str());
	data.push_str(format!("view.style.comment1=color\\:\\{}\n", &scheme.comment1).as_str());
	data.push_str(format!("view.style.digit=color\\:\\{}\n", 	&scheme.digit).as_str());
	data.push_str(format!("view.style.operator=color\\:\\{}\n", &scheme.operator).as_str());
	data.push_str(format!("view.style.function=color\\:\\{}\n", &scheme.function).as_str());
	data.push_str(format!("view.style.literal1=color\\:\\{}\n", &scheme.literal1).as_str());
	data.push_str(format!("view.style.literal2=color\\:\\{}\n", &scheme.literal2).as_str());
	data.push_str(format!("view.style.literal3=color\\:\\{}\n", &scheme.literal3).as_str());
	data.push_str(format!("view.style.caretColor=color\\:\\{}\n", &scheme.caretColor).as_str());
	data.push_str(format!("view.style.selectionColor=color\\:\\{}\n", &scheme.selectionColor).as_str());
	data.push_str(format!("view.style.eolMarkerColor=color\\:\\{}\n", &scheme.eolMarkerColor).as_str());
	data.push_str(format!("view.style.lineHighlightColor=color\\:\\{}\n", &scheme.lineHighlightColor).as_str());

	println!("{}", data);
	write_file(&"converted.jedit-scheme".to_string(), &data)
}