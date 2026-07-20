mod array {
    pub mod easy_14;
    pub mod easy_217;
    pub mod easy_27;
    pub mod med_11;
}

use std::env;
use std::process;

fn main() {
    let module_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: cargo run -- <module_name>");
        process::exit(1);
    });

    match module_name.as_str() {
        "easy_14" => array::easy_14::easy_14(),
        "easy_27" => array::easy_27::easy_27(),
        "easy_217" => array::easy_217::easy_217(),
        "med_11" => array::med_11::med_11(),
        _ => {
            eprintln!("unknown module: {module_name}");
            process::exit(1);
        }
    };
}
