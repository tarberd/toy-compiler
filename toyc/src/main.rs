use llvm_sys as llvm;
use std::ffi::CString;
use toy_parser::ast::Ast;

fn main() {
    match std::env::args().nth(1) {
        Some(input_file_name) => {
            println!("Compiling file: {}", input_file_name);
            compile(&input_file_name);
        }
        None => println!("Not file provided. Nothing to do."),
    }
}

fn compile(input_file_name: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use toy_parser::parser;

    let mut file = File::open(input_file_name)
        .expect(format!("Failed to open file: {}.", input_file_name).as_str());

    let mut file_buffer = String::new();

    file.read_to_string(&mut file_buffer)
        .expect(format!("Failed to read file: {}.", input_file_name).as_str());

    match parser::ModuleParser::new().parse(&file_buffer) {
        Ok(ast) => {
            println!("Finish parsing file.");

            let llvm_module = unsafe {
                llvm::core::LLVMModuleCreateWithName(
                    CString::new(input_file_name)
                        .expect(
                            format!(
                                "Failed to create CString from file name: {}",
                                input_file_name
                            )
                            .as_str(),
                        )
                        .as_ptr(),
                )
            };

            ast_to_llvm_module(&ast, llvm_module);

            unsafe {
                llvm::bit_writer::LLVMWriteBitcodeToFile(
                    llvm_module,
                    CString::new(format!("{}.bc", input_file_name))
                        .expect("Failed to create CString from output file name: {}")
                        .as_ptr(),
                );
            };

            unsafe {
                llvm::core::LLVMDisposeModule(llvm_module);
            };
        }
        Err(error) => println!("Parsing erro: {:#?}", error),
    }
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
                    llvm::core::LLVMVoidType(),
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
