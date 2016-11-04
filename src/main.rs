mod profiles;
mod ir;

use ir::*;

fn main() {
    println!("A tool to convert color schemes between editors.");

    ir::decode_from_ir("src/template_ir.json".to_string());
}