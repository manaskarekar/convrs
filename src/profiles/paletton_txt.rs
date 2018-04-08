use ir::{read_file, IntermediateRepr};

macro_rules! paletton {

	($lines:expr, $line_number:expr, $col_number:expr) => ({
		$lines[$line_number].split(' ').collect::<Vec<&str>>()[$col_number].to_string()
	});

	($lines:expr, $line_number:expr) => ({
		paletton!($lines, $line_number, 6) // Hardcoded to read the 7th element, can genericize if needed
	});
}


pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading paletton_txt color palette..");

	let scheme_data = read_file(&infile);
	let scheme_name = infile.split('.').collect::<Vec<&str>>()[0]; //split on the first '.' and use the first value for name
	let lines = scheme_data.lines().collect::<Vec<&str>>();


	//TODO: Strip all newlines, right now, when copy pasting from text output, include the first newline.
	//TODO: Do this right, quick and dirty for now.
	//TODO: Come back and do index-based (enumerate + find) to navigate the iterator.
	//println!("{:#?}", &lines.next());
	//	assert_eq!(&lines.next(), &Some("*** Primary color:"));
	//	assert line !empty and index <= len

	println!("{:#?}", &lines);

	let scheme = IntermediateRepr { //create new from scheme_data
		name : scheme_name.to_string(),

		//fgcolor : lines[12].split(' ').collect::<Vec<&str>>()[6].to_string(),
		//bgcolor : lines[31].split(' ').collect::<Vec<&str>>()[6].to_string(),
		fgcolor :				"#000000".to_string(),
		bgcolor :				"#FFFFFF".to_string(),

		keyword1 :				paletton!(lines,7),
		keyword2 :				paletton!(lines,7), //TODO: Temporarily using keyword1 val
		keyword3 :				paletton!(lines,7),
		keyword4 :				paletton!(lines,7),

		comment1 :				paletton!(lines,18),
		digit	 :				paletton!(lines,25),
		operator :				paletton!(lines,31),
		function :				paletton!(lines,34),

		literal1 :				paletton!(lines,31),
		literal2 :				paletton!(lines,31),
		literal3 :				paletton!(lines,31),
		caretColor :			paletton!(lines,31),
		selectionColor :		paletton!(lines,23),
		eolMarkerColor :		paletton!(lines,23),
		lineHighlightColor :	paletton!(lines,24),

	/*
	let primary_shade0 = &lines[8].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade1 = &lines[9].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade2 = &lines[10].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade3 = &lines[11].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade4 = &lines[12].split(' ').collect::<Vec<&str>>()[6];

	let secondary1_shade0 = &lines[15].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade1 = &lines[16].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade2 = &lines[17].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade3 = &lines[18].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade4 = &lines[19].split(' ').collect::<Vec<&str>>()[6];

	let secondary2_shade0 = &lines[23].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade1 = &lines[24].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade2 = &lines[25].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade3 = &lines[26].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade4 = &lines[27].split(' ').collect::<Vec<&str>>()[6];

	let complement_shade0 = &lines[31].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade1 = &lines[32].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade2 = &lines[33].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade3 = &lines[34].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade4 = &lines[35].split(' ').collect::<Vec<&str>>()[6];

	*/

	};


	println!("paletton scheme: {:#?}", &scheme);

	scheme
}

pub fn write_scheme() {}
