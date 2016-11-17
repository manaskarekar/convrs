extern crate rustc_serialize;

mod profiles;
mod ir;

use ir::*;

fn main() {
	//println!("A tool to convert color schemes between editors.");

	//let json_data = ir::read_json2("src/test.json".to_string());
	let json_data = ir::read_json("src/test.json".to_string());
	//println!("{:#?}", &json_data);
	let ir_obj = ir::json_to_ir(&json_data);
	//let x:
}