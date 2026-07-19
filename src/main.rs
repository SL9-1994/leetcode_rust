mod array {
    pub mod easy_14;
}

use std::env;
use std::process;

fn main() {
    let module_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: cargo run -- <module_name>");
        process::exit(1);
    });

    let result = match module_name.as_str() {
        "easy_14" => array::easy_14::easy_14(),
        _ => {
            eprintln!("unknown module: {module_name}");
            process::exit(1);
        }
    };

    println!("{result}");
}
