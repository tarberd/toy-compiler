use llvm_sys as llvm;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use toy_parser::ast::{Ast, Expression, Operator};

#[derive(Debug, structopt::StructOpt)]
pub struct Config {
    #[structopt(parse(from_os_str))]
    pub file: std::path::PathBuf,

    #[structopt(short, long)]
    pub emit_llvm_ir: bool,

    #[structopt(short = "a", long)]
    pub emit_ast: bool,
}

pub fn drive(config: Config) {
    use std::fs::File;
    use std::io::prelude::Read;
    use toy_parser::parser::ModuleParser;

    if !config.file.is_file() {
        panic!("Please provide a valid file");
    }

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

    if config.emit_ast {
        println!("{:#?}", ast);
    }

    let llvm_module = unsafe {
        let c_module_name =
            CString::new(config.file.file_name().unwrap().to_str().unwrap()).unwrap();

        llvm::core::LLVMModuleCreateWithName(c_module_name.as_ptr())
    };

    let mut symbol_table: HashMap<String, *mut llvm::LLVMValue> = HashMap::new();
    ast_to_llvm_module(&ast, llvm_module, &mut symbol_table);

    if config.emit_llvm_ir {
        let asm =
            unsafe { std::ffi::CStr::from_ptr(llvm::core::LLVMPrintModuleToString(llvm_module)) };

        println!("{}", asm.to_str().expect("c string error on output"));
    }

    unsafe {
        let target_triple = llvm::target_machine::LLVMGetDefaultTargetTriple();
        let target_cpu = llvm::target_machine::LLVMGetHostCPUName();
        let target_features = llvm::target_machine::LLVMGetHostCPUFeatures();

        if let 1 = llvm::target::LLVM_InitializeNativeTarget() {
            panic!("Failed to initialize llvm native target");
        };

        if let 1 = llvm::target::LLVM_InitializeNativeAsmPrinter() {
            panic!("Failed to initialize llvm native target");
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

        if let 1 = llvm::target_machine::LLVMTargetMachineEmitToFile(
            target_machine,
            llvm_module,
            ptr,
            codegen,
            err,
        ) {
            panic!("Failed to initialize llvm native target");
        };
    };

    unsafe {
        llvm::core::LLVMDisposeModule(llvm_module);
    };
}

fn ast_to_llvm_module(
    ast: &Ast,
    module: *mut llvm::LLVMModule,
    symbol_table: &mut HashMap<String, *mut llvm::LLVMValue>,
) {
    match ast {
        Ast::Module { contents } => {
            for content in contents {
                ast_to_llvm_module(content, module, symbol_table);
            }
        }
        Ast::FunctionDeclaration {
            id,
            body,
            parameters: args,
        } => {
            let function_type = unsafe {
                let mut parameter_types = vec![llvm::core::LLVMInt32Type(); args.len()];

                llvm::core::LLVMFunctionType(
                    llvm::core::LLVMInt32Type(),
                    parameter_types.as_mut_ptr(),
                    parameter_types.len() as std::os::raw::c_uint,
                    0,
                )
            };

            let function = unsafe {
                let c_name = CString::new(id.as_str()).unwrap();

                llvm::core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
            };

            symbol_table.insert(id.clone(), function);

            unsafe {
                let function_params = Vec::with_capacity(args.len());

                let mut function_params = std::mem::ManuallyDrop::new(function_params);

                llvm::core::LLVMGetParams(function, function_params.as_mut_ptr());

                let function_params = Vec::from_raw_parts(
                    function_params.as_mut_ptr(),
                    function_params.capacity(),
                    function_params.capacity(),
                );

                for (value, name) in function_params.iter().zip(args.iter()) {
                    symbol_table.insert(name.clone(), *value);

                    let c_name = CString::new(name.as_str()).unwrap();
                    let c_name_len = c_name.as_bytes().len();

                    llvm::core::LLVMSetValueName2(*value, c_name.as_ptr(), c_name_len);
                }
            }

            let function_block = unsafe {
                llvm::core::LLVMAppendBasicBlock(
                    function,
                    b"body\0".as_ptr() as *const std::os::raw::c_char,
                )
            };

            let builder = unsafe { llvm::core::LLVMCreateBuilder() };
            unsafe {
                llvm::core::LLVMPositionBuilderAtEnd(builder, function_block);
                llvm::core::LLVMBuildRet(builder, const_expression_to_llvm_valueref(body, builder, symbol_table));
                llvm::core::LLVMDisposeBuilder(builder);
            }
        }
        _ => (),
    };
}

fn const_expression_to_llvm_valueref(
    expression: &Expression,
    builder: *mut llvm::LLVMBuilder,
    symbol_table: &HashMap<String, *mut llvm::LLVMValue>,
) -> *mut llvm::LLVMValue {
    match expression {
        Expression::IntegerLiteral { value } => unsafe {
            llvm::core::LLVMConstInt(
                llvm::core::LLVMInt32Type(),
                *value as std::os::raw::c_ulonglong,
                0 as std::os::raw::c_int,
            )
        },
        Expression::Identifier { id } => symbol_table[id],
        Expression::Unary {
            operator,
            expression,
        } => match operator {
            Operator::Neg => unsafe {
                llvm::core::LLVMBuildNeg(
                    builder,
                    const_expression_to_llvm_valueref(expression, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"neg_tmp\0").as_ptr(),
                )
            },
            _ => panic!("{:?} is not a unary operator", operator),
        },
        Expression::Binary {
            operator,
            left,
            right,
        } => match operator {
            Operator::Plus => unsafe {
                llvm::core::LLVMBuildAdd(
                    builder,
                    const_expression_to_llvm_valueref(left, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"add_tmp\0").as_ptr(),
                )
            },
            Operator::Minus => unsafe {
                llvm::core::LLVMBuildSub(
                    builder,
                    const_expression_to_llvm_valueref(left, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"sub_tmp\0").as_ptr(),
                )
            },
            Operator::Mul => unsafe {
                llvm::core::LLVMBuildMul(
                    builder,
                    const_expression_to_llvm_valueref(left, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"mul_tmp\0").as_ptr(),
                )
            },
            Operator::Div => unsafe {
                llvm::core::LLVMBuildSDiv(
                    builder,
                    const_expression_to_llvm_valueref(left, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"div_tmp\0").as_ptr(),
                )
            },
            _ => panic!("{:?} is not a binary operator.", operator),
        },
        Expression::Block { return_expression } => {
            const_expression_to_llvm_valueref(return_expression, builder, symbol_table)
        }
        Expression::Call {
            id,
            arguments,
        } => unsafe {
            let mut arguments: Vec<_> = arguments.iter().map(|arg| const_expression_to_llvm_valueref(arg, builder, symbol_table)).collect();
            llvm::core::LLVMBuildCall(builder, symbol_table[id], arguments.as_mut_ptr(), arguments.len() as u32, CStr::from_bytes_with_nul_unchecked(b"call_tmp\0").as_ptr())
        },
    }
}
