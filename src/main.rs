mod profiles;
mod ir;

use ir::*;

fn main() {
    println!("A tool to convert color schemes between editors.");

    ir::read_json_to_ir("src/template_ir.json".to_string());
}