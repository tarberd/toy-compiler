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

pub struct SymbolTable {
    table_stack: Vec<HashMap<String, *mut llvm::LLVMValue>>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            table_stack: vec![HashMap::new()],
        }
    }

    fn contains_symbol(&self, id: &String) -> bool {
        self.table_stack
            .iter()
            .rev()
            .find_map(|table| Some(table.contains_key(id)))
            .unwrap()
    }

    fn insert(&mut self, id: String, value: *mut llvm::LLVMValue) {
        self.table_stack.last_mut().unwrap().insert(id, value);
    }

    fn push(&mut self) {
        self.table_stack.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.table_stack.pop();
    }
}

impl std::ops::Index<&String> for SymbolTable {
    type Output = *mut llvm::LLVMValue;

    fn index(&self, string: &String) -> &Self::Output {
        self.table_stack
            .iter()
            .rev()
            .find_map(|table| table.get(string))
            .unwrap()
    }
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

    let builder = unsafe { llvm::core::LLVMCreateBuilder() };
    ast_to_llvm_module(&ast, llvm_module, builder, &mut SymbolTable::new());
    unsafe {
        llvm::core::LLVMDisposeBuilder(builder);
    };

    if config.emit_llvm_ir {
        let asm =
            unsafe { std::ffi::CStr::from_ptr(llvm::core::LLVMPrintModuleToString(llvm_module)) };

        println!("{}", asm.to_str().expect("c string error on output"));
    }

    let target_triple = unsafe { llvm::target_machine::LLVMGetDefaultTargetTriple() };
    let target_cpu = unsafe { llvm::target_machine::LLVMGetHostCPUName() };
    let target_features = unsafe { llvm::target_machine::LLVMGetHostCPUFeatures() };

    if let 1 = unsafe { llvm::target::LLVM_InitializeNativeTarget() } {
        panic!("Failed to initialize llvm native target");
    };

    if let 1 = unsafe { llvm::target::LLVM_InitializeNativeAsmPrinter() } {
        panic!("Failed to initialize llvm native target");
    };

    let llvm_target = unsafe { llvm::target_machine::LLVMGetFirstTarget() };

    let opt_level = llvm::target_machine::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault;
    let reloc_mode = llvm::target_machine::LLVMRelocMode::LLVMRelocDefault;
    let code_model = llvm::target_machine::LLVMCodeModel::LLVMCodeModelDefault;

    let target_machine = unsafe {
        llvm::target_machine::LLVMCreateTargetMachine(
            llvm_target,
            target_triple,
            target_cpu,
            target_features,
            opt_level,
            reloc_mode,
            code_model,
        )
    };

    let mut object_name = String::from(config.file.file_name().unwrap().to_str().unwrap());
    object_name.push_str(".o");

    let object_name = CString::new(object_name).unwrap();

    let codegen = llvm::target_machine::LLVMCodeGenFileType::LLVMObjectFile;

    let err = std::ptr::null_mut();

    if let 1 = unsafe {
        llvm::target_machine::LLVMTargetMachineEmitToFile(
            target_machine,
            llvm_module,
            object_name.as_ptr() as *mut std::os::raw::c_char,
            codegen,
            err,
        )
    } {
        unsafe { llvm::core::LLVMDisposeMessage(*err) };
        panic!("Failed to initialize llvm native target");
    };

    unsafe { llvm::target_machine::LLVMDisposeTargetMachine(target_machine) };

    unsafe { llvm::core::LLVMDisposeModule(llvm_module) };
}

fn ast_to_llvm_module(
    ast: &Ast,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder,
    symbol_table: &mut SymbolTable,
) {
    match ast {
        Ast::Module { contents } => {
            for content in {
                contents.iter().filter(|x| match x {
                    Ast::VariableDefinition {
                        id: _,
                        expression: _,
                    } => false,
                    _ => true,
                })
            } {
                ast_to_llvm_module(content, module, builder, symbol_table);
            }
        }
        Ast::FunctionDeclaration { id, parameters } => {
            let function_type = unsafe {
                let mut parameter_types = vec![llvm::core::LLVMInt32Type(); parameters.len()];

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
        }
        Ast::FunctionDefinition {
            id,
            parameters,
            body,
        } => {
            let function = if !symbol_table.contains_symbol(id) {
                let function_type = unsafe {
                    let mut parameter_types = vec![llvm::core::LLVMInt32Type(); parameters.len()];

                    llvm::core::LLVMFunctionType(
                        llvm::core::LLVMInt32Type(),
                        parameter_types.as_mut_ptr(),
                        parameter_types.len() as std::os::raw::c_uint,
                        0,
                    )
                };

                unsafe {
                    let c_name = CString::new(id.as_str()).unwrap();

                    llvm::core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
                }
            } else {
                symbol_table[id]
            };

            symbol_table.insert(id.clone(), function);

            let function_params = unsafe {
                let function_params = Vec::with_capacity(parameters.len());

                let mut function_params = std::mem::ManuallyDrop::new(function_params);

                llvm::core::LLVMGetParams(function, function_params.as_mut_ptr());

                Vec::from_raw_parts(
                    function_params.as_mut_ptr(),
                    function_params.capacity(),
                    function_params.capacity(),
                )
            };

            symbol_table.push();

            for (value, name) in function_params.iter().zip(parameters.iter()) {
                symbol_table.insert(name.clone(), *value);

                let c_name = CString::new(name.as_str()).unwrap();
                let c_name_len = c_name.as_bytes().len();

                unsafe { llvm::core::LLVMSetValueName2(*value, c_name.as_ptr(), c_name_len) }
            }

            let function_block = unsafe {
                llvm::core::LLVMAppendBasicBlock(
                    function,
                    b"body\0".as_ptr() as *const std::os::raw::c_char,
                )
            };

            unsafe {
                llvm::core::LLVMPositionBuilderAtEnd(builder, function_block);
                llvm::core::LLVMBuildRet(
                    builder,
                    const_expression_to_llvm_valueref(body, module, builder, symbol_table),
                );
            };

            symbol_table.pop();
        }
        Ast::VariableDefinition { id, expression } => {
            let c_id = CString::new(id.as_str()).unwrap();

            let l_value = unsafe {
                llvm::core::LLVMBuildAlloca(builder, llvm::core::LLVMInt32Type(), c_id.as_ptr())
            };

            symbol_table.insert(id.clone(), l_value);

            let r_value =
                const_expression_to_llvm_valueref(expression, module, builder, symbol_table);

            unsafe {
                llvm::core::LLVMBuildStore(builder, r_value, l_value);
            };
        }
        _ => (),
    };
}

fn const_expression_to_llvm_valueref(
    expression: &Expression,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder,
    symbol_table: &mut SymbolTable,
) -> *mut llvm::LLVMValue {
    match expression {
        Expression::IntegerLiteral { value } => unsafe {
            llvm::core::LLVMConstInt(
                llvm::core::LLVMInt32Type(),
                *value as std::os::raw::c_ulonglong,
                0 as std::os::raw::c_int,
            )
        },
        Expression::Identifier { id } => unsafe {
            match llvm::core::LLVMGetValueKind(symbol_table[id]) {
                llvm::LLVMValueKind::LLVMArgumentValueKind => symbol_table[id],
                _ => {
                    let c_id = CString::new(id.as_str()).unwrap();
                    llvm::core::LLVMBuildLoad(builder, symbol_table[id], c_id.as_ptr())
                }
            }
        },
        Expression::Unary {
            operator,
            expression,
        } => match operator {
            Operator::Neg => unsafe {
                llvm::core::LLVMBuildNeg(
                    builder,
                    const_expression_to_llvm_valueref(expression, module, builder, symbol_table),
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
                    const_expression_to_llvm_valueref(left, module, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, module, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"add_tmp\0").as_ptr(),
                )
            },
            Operator::Minus => unsafe {
                llvm::core::LLVMBuildSub(
                    builder,
                    const_expression_to_llvm_valueref(left, module, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, module, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"sub_tmp\0").as_ptr(),
                )
            },
            Operator::Mul => unsafe {
                llvm::core::LLVMBuildMul(
                    builder,
                    const_expression_to_llvm_valueref(left, module, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, module, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"mul_tmp\0").as_ptr(),
                )
            },
            Operator::Div => unsafe {
                llvm::core::LLVMBuildSDiv(
                    builder,
                    const_expression_to_llvm_valueref(left, module, builder, symbol_table),
                    const_expression_to_llvm_valueref(right, module, builder, symbol_table),
                    CStr::from_bytes_with_nul_unchecked(b"div_tmp\0").as_ptr(),
                )
            },
            _ => panic!("{:?} is not a binary operator.", operator),
        },
        Expression::Block {
            statements,
            return_expression,
        } => {
            for statement in statements {
                ast_to_llvm_module(statement, module, builder, symbol_table);
            }
            const_expression_to_llvm_valueref(return_expression, module, builder, symbol_table)
        }
        Expression::Call { id, arguments } => unsafe {
            let mut arguments: Vec<_> = arguments
                .iter()
                .map(|arg| const_expression_to_llvm_valueref(arg, module, builder, symbol_table))
                .collect();
            llvm::core::LLVMBuildCall(
                builder,
                symbol_table[id],
                arguments.as_mut_ptr(),
                arguments.len() as u32,
                CStr::from_bytes_with_nul_unchecked(b"call_tmp\0").as_ptr(),
            )
        },
    }
}
