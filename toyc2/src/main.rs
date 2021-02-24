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
            Ok(source) => compile(
                source,
                file_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string() ,
            ),
            Err(err) => println!("{}", err),
        }
    }
}

fn compile(source: String, file_name: String) {
    if let Some(module) = toy_parser2::parse_root_module(&source) {
        let _ = toy_typecheck::typecheck_root_module(&source, &module);

        let mut lowerer = toy_backend::Lower::new(&source, file_name);
        let llvm_module = lowerer.lower_root_module(&module);

        toy_backend::print_module(llvm_module);

        let backend = toy_backend::Backend::new();
        backend.emit_llvm(llvm_module);
    }
}
