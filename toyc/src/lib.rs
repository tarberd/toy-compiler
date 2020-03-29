use llvm_sys as llvm;
use std::ffi::CString;
use toy_parser::ast::Ast;

#[derive(Debug, structopt::StructOpt)]
pub struct Config {
    #[structopt(parse(from_os_str))]
    pub file: std::path::PathBuf,

    #[structopt(short, long)]
    pub emit_llvm_ir: bool,
}

pub fn drive(config: Config) {
    use std::fs::File;
    use std::io::prelude::Read;
    use toy_parser::parser::ModuleParser;

    let file_buffer = match File::open(&config.file) {
        Ok(mut file) => {
            let mut file_buffer = String::new();

            match file.read_to_string(&mut file_buffer) {
                Ok(_) => (),
                Err(e) => {
                    println!(
                        "Failed to read file {}: {}",
                        (&config.file).to_str().unwrap(),
                        e
                    );
                    return;
                }
            };

            file_buffer
        }
        Err(e) => {
            println!(
                "Error trying to open file {}: {}",
                config.file.to_str().unwrap(),
                e
            );
            return;
        }
    };

    let ast = ModuleParser::new().parse(&file_buffer).unwrap();

    let llvm_module = unsafe {
        use std::os::unix::ffi::OsStringExt;
        llvm::core::LLVMModuleCreateWithName(
            CString::new(config.file.file_name().unwrap().to_owned().into_vec())
                .expect("Failed to create CString.")
                .as_ptr(),
        )
    };

    ast_to_llvm_module(&ast, llvm_module);

    if config.emit_llvm_ir {
        let asm =
            unsafe { std::ffi::CStr::from_ptr(llvm::core::LLVMPrintModuleToString(llvm_module)) };

        println!("{}", asm.to_str().expect("c string error on output"));
    }

    unsafe {
        llvm::bit_writer::LLVMWriteBitcodeToFile(
            llvm_module,
            CString::new(format!(
                "{}.bc",
                config.file.file_name().unwrap().to_str().unwrap()
            ))
            .expect("Failed to create CString from output file name: {}")
            .as_ptr(),
        );
    };

    unsafe {
        llvm::core::LLVMDisposeModule(llvm_module);
    };
}

fn ast_to_llvm_module(ast: &Ast, module: *mut llvm::LLVMModule) {
    match ast {
        Ast::Module { contents } => {
            ast_to_llvm_module(contents, module);
        }
        Ast::Function(function_declaration) => {
            let function_type = unsafe {
                let mut parameter_types = [];

                llvm::core::LLVMFunctionType(
                    llvm::core::LLVMInt32Type(),
                    parameter_types.as_mut_ptr(),
                    parameter_types.len() as std::os::raw::c_uint,
                    0,
                )
            };

            let function = unsafe {
                llvm::core::LLVMAddFunction(
                    module,
                    CString::new(function_declaration.id.clone())
                        .unwrap()
                        .as_ptr(),
                    function_type,
                )
            };

            let function_block = unsafe {
                llvm::core::LLVMAppendBasicBlock(
                    function,
                    b"body\0".as_ptr() as *const std::os::raw::c_char,
                )
            };

            let builder = unsafe { llvm::core::LLVMCreateBuilder() };

            unsafe {
                llvm::core::LLVMPositionBuilderAtEnd(builder, function_block);
                llvm::core::LLVMBuildRet(
                    builder,
                    llvm::core::LLVMConstInt(
                        llvm::core::LLVMInt32Type(),
                        function_declaration.body.return_expression as std::os::raw::c_ulonglong,
                        0 as std::os::raw::c_int,
                    ),
                );
                llvm::core::LLVMDisposeBuilder(builder);
            }
        }
        _ => (),
    };
}
