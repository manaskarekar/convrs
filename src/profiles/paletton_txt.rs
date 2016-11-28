use ir::{read_file, tokenize, IntermediateRepr};

pub fn read_scheme(infile: &String) -> IntermediateRepr {
	println!("Reading paletton_txt color palette..");

	let mut scheme = IntermediateRepr{}; //create new from scheme_data
	let scheme_data = read_file(&infile);
	let mut lines = scheme_data.lines().collect::<Vec<&str>>();

	//TODO: Do this right, quick and dirty for now.
	//TODO: Come back and do index-based (enumerate + find) to navigate the iterator.
	//println!("{:#?}", &lines.next());
	//	assert_eq!(&lines.next(), &Some("*** Primary color:"));
	//	assert line !empty and index <= len

	let primary_shade0 = &lines[8].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade1 = &lines[9].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade2 = &lines[10].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade3 = &lines[11].split(' ').collect::<Vec<&str>>()[6];
	let primary_shade4 = &lines[12].split(' ').collect::<Vec<&str>>()[6];

	let secondary1_shade0 = &lines[16].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade1 = &lines[17].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade2 = &lines[18].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade3 = &lines[19].split(' ').collect::<Vec<&str>>()[6];
	let secondary1_shade4 = &lines[20].split(' ').collect::<Vec<&str>>()[6];

	let secondary2_shade0 = &lines[24].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade1 = &lines[25].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade2 = &lines[26].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade3 = &lines[27].split(' ').collect::<Vec<&str>>()[6];
	let secondary2_shade4 = &lines[28].split(' ').collect::<Vec<&str>>()[6];

	let complement_shade0 = &lines[32].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade1 = &lines[33].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade2 = &lines[34].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade3 = &lines[35].split(' ').collect::<Vec<&str>>()[6];
	let complement_shade4 = &lines[36].split(' ').collect::<Vec<&str>>()[6];

	scheme
}

pub fn write_scheme() {}