use llvm_sys as llvm;
use std::ffi::CString;
use toy_parser::ast::{Ast, Expression, Operator};

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

    println!("{:#?}", ast);

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
        let target_triple = llvm::target_machine::LLVMGetDefaultTargetTriple();
        let target_cpu = llvm::target_machine::LLVMGetHostCPUName();
        let target_features = llvm::target_machine::LLVMGetHostCPUFeatures();

        match llvm::target::LLVM_InitializeNativeTarget() {
            1 => {
                println!("Failed to initialize llvm native target");
                return;
            }
            _ => (),
        };

        match llvm::target::LLVM_InitializeNativeAsmPrinter() {
            1 => {
                println!("Failed to initialize llvm native target");
                return;
            }
            _ => (),
        };

        let llvm_target = llvm::target_machine::LLVMGetFirstTarget();

        let opt_level = llvm::target_machine::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault;
        let reloc_mode = llvm::target_machine::LLVMRelocMode::LLVMRelocDefault;
        let code_model = llvm::target_machine::LLVMCodeModel::LLVMCodeModelDefault;

        let target_machine = llvm::target_machine::LLVMCreateTargetMachine(
            llvm_target,
            target_triple,
            target_cpu,
            target_features,
            opt_level,
            reloc_mode,
            code_model,
        );

        let mut x: Vec<std::os::raw::c_char> = vec!['a', 'b', 'c', '\0']
            .into_iter()
            .map(|x| x as std::os::raw::c_char)
            .collect();
        let slice = x.as_mut_slice();
        let ptr = slice.as_mut_ptr();

        let codegen = llvm::target_machine::LLVMCodeGenFileType::LLVMObjectFile;

        let err: *mut *mut std::os::raw::c_char = [].as_mut_ptr();

        match llvm::target_machine::LLVMTargetMachineEmitToFile(
            target_machine,
            llvm_module,
            ptr,
            codegen,
            err,
        ) {
            1 => {
                println!("Failed to initialize llvm native target");
                return;
            }
            _ => (),
        }
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
        // Ast::Expression(expr) => {
        //     expression_to_llvm_module(expr, module);
        // }
        Ast::FunctionDeclaration {
            id,
            body,
            args: _args,
        } => {
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
                    CString::new(id.clone()).unwrap().as_ptr(),
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
                llvm::core::LLVMBuildRet(builder, const_expression_to_llvm_valueref(body));
                llvm::core::LLVMDisposeBuilder(builder);
            }
        }
        _ => (),
    };
}

fn const_expression_to_llvm_valueref(expression: &Expression) -> *mut llvm::LLVMValue {
    match expression {
        Expression::IntegerLiteral { value } => unsafe {
            llvm::core::LLVMConstInt(
                llvm::core::LLVMInt32Type(),
                *value as std::os::raw::c_ulonglong,
                0 as std::os::raw::c_int,
            )
        },
        Expression::Unary {
            operator,
            expression,
        } => match operator {
            Operator::Minus => unsafe {
                llvm::core::LLVMConstNeg(const_expression_to_llvm_valueref(expression))
            },
            _ => panic!("only minus is a unary operator"),
        },
        Expression::Binary {
            operator,
            left,
            right,
        } => match operator {
            Operator::Plus => unsafe {
                llvm::core::LLVMConstAdd(
                    const_expression_to_llvm_valueref(left),
                    const_expression_to_llvm_valueref(right),
                )
            },
            Operator::Minus => unsafe {
                llvm::core::LLVMConstSub(
                    const_expression_to_llvm_valueref(left),
                    const_expression_to_llvm_valueref(right),
                )
            },
            Operator::Mul => unsafe {
                llvm::core::LLVMConstMul(
                    const_expression_to_llvm_valueref(left),
                    const_expression_to_llvm_valueref(right),
                )
            },
            Operator::Div => unsafe {
                llvm::core::LLVMConstSDiv(
                    const_expression_to_llvm_valueref(left),
                    const_expression_to_llvm_valueref(right),
                )
            },
        },
        Expression::Block {
            return_expression,
        } => {
            const_expression_to_llvm_valueref(return_expression)
        }
    }
}
