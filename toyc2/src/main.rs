use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<_> = std::env::args()
        .into_iter()
        .map(|arg| PathBuf::from(arg))
        .collect();

    if let Some(file_path) = args.iter().nth(1) {
        println!("Compiling file: {}", file_path.to_str().unwrap());
        match fs::read_to_string(file_path) {
            Ok(source) => compile(source),
            Err(err) => println!("{}", err),
        }
    }
}

fn compile(source: String) {
    if let Some(module) = toy_parser2::parse_root_module(&source) {
        let _ = toy_typecheck::typecheck_root_module(&source, &module);

        toy_backend::lower_to_llvm()
    }
}
