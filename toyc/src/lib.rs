mod backend;

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
    use llvm_sys as llvm;
    use std::ffi::{CStr, CString};
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

    let module = ModuleParser::new().parse(&file_buffer).unwrap();
    let module = toy_parser::typecheck::typecheck_root_module(module);

    if config.emit_ast {
        println!("{:#?}", module);
    }

    if let 1 = unsafe { llvm::target::LLVM_InitializeNativeTarget() } {
        panic!("Failed to initialize llvm native target");
    };

    if let 1 = unsafe { llvm::target::LLVM_InitializeNativeAsmPrinter() } {
        panic!("Failed to initialize llvm native target");
    };

    let target_triple = unsafe { llvm::target_machine::LLVMGetDefaultTargetTriple() };
    let target_cpu = unsafe { llvm::target_machine::LLVMGetHostCPUName() };
    let target_features = unsafe { llvm::target_machine::LLVMGetHostCPUFeatures() };

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

    let llvm_the_context = unsafe { llvm::core::LLVMContextCreate() };

    let llvm_module = unsafe {
        let c_module_name =
            CString::new(config.file.file_name().unwrap().to_str().unwrap()).unwrap();

        llvm::core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), llvm_the_context)
    };

    unsafe {
        llvm::target::LLVMSetModuleDataLayout(
            llvm_module,
            llvm::target_machine::LLVMCreateTargetDataLayout(target_machine),
        );
    }

    backend::populate_llvm_module(llvm_module, module);

    if config.emit_llvm_ir {
        let asm =
            unsafe { std::ffi::CStr::from_ptr(llvm::core::LLVMPrintModuleToString(llvm_module)) };

        println!("{}", asm.to_str().expect("c string error on output"));
    }

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


// fn ast_to_llvm_module(
//     ast: &Ast,
//     module: *mut llvm::LLVMModule,
//     builder: *mut llvm::LLVMBuilder,
//     symbol_table: &mut SymbolTable,
// ) {
//     match ast {
//         Ast::Module { contents } => {
//             for content in {
//                 contents.iter().filter(|x| match x {
//                     Ast::VariableDefinition {
//                         type_id: _,
//                         id: _,
//                         expression: _,
//                     } => false,
//                     _ => true,
//                 })
//             } {
//                 ast_to_llvm_module(content, module, builder, symbol_table);
//             }
//         }
//         Ast::FunctionDeclaration { id, parameters } => {
//             let function_type = unsafe {
//                 let mut parameter_types = vec![llvm::core::LLVMInt32Type(); parameters.len()];
//
//                 llvm::core::LLVMFunctionType(
//                     llvm::core::LLVMInt32Type(),
//                     parameter_types.as_mut_ptr(),
//                     parameter_types.len() as std::os::raw::c_uint,
//                     0,
//                 )
//             };
//
//             let function = unsafe {
//                 let c_name = CString::new(id.as_str()).unwrap();
//
//                 llvm::core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
//             };
//
//             symbol_table.insert(id.clone(), function);
//         }
//         Ast::FunctionDefinition {
//             id,
//             parameters,
//             body,
//         } => {
//             let function = if !symbol_table.contains_symbol(id) {
//                 let function_type = unsafe {
//                     let mut parameter_types = vec![llvm::core::LLVMInt32Type(); parameters.len()];
//
//                     llvm::core::LLVMFunctionType(
//                         llvm::core::LLVMInt32Type(),
//                         parameter_types.as_mut_ptr(),
//                         parameter_types.len() as std::os::raw::c_uint,
//                         0,
//                     )
//                 };
//
//                 unsafe {
//                     let c_name = CString::new(id.as_str()).unwrap();
//
//                     llvm::core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
//                 }
//             } else {
//                 symbol_table[id]
//             };
//
//             symbol_table.insert(id.clone(), function);
//
//             let function_params = unsafe {
//                 let function_params = Vec::with_capacity(parameters.len());
//
//                 let mut function_params = std::mem::ManuallyDrop::new(function_params);
//
//                 llvm::core::LLVMGetParams(function, function_params.as_mut_ptr());
//
//                 Vec::from_raw_parts(
//                     function_params.as_mut_ptr(),
//                     function_params.capacity(),
//                     function_params.capacity(),
//                 )
//             };
//
//             symbol_table.push();
//
//             for (value, name) in function_params.iter().zip(parameters.iter().map(|x| x.0.clone())) {
//                 symbol_table.insert(name.clone(), *value);
//
//                 let c_name = CString::new(name.as_str()).unwrap();
//                 let c_name_len = c_name.as_bytes().len();
//
//                 unsafe { llvm::core::LLVMSetValueName2(*value, c_name.as_ptr(), c_name_len) }
//             }
//
//             let function_block = unsafe {
//                 llvm::core::LLVMAppendBasicBlock(
//                     function,
//                     b"body\0".as_ptr() as *const std::os::raw::c_char,
//                 )
//             };
//
//             unsafe {
//                 llvm::core::LLVMPositionBuilderAtEnd(builder, function_block);
//                 llvm::core::LLVMBuildRet(
//                     builder,
//                     const_expression_to_llvm_valueref(
//                         body,
//                         module,
//                         builder,
//                         symbol_table,
//                         &mut Context::new(),
//                     ),
//                 );
//             };
//
//             symbol_table.pop();
//         }
//         Ast::VariableDefinition {
//             type_id,
//             id,
//             expression,
//         } => {
//             let c_id = CString::new(id.as_str()).unwrap();
//
//             let l_value = match type_id {
//                 Type::I32 | Type::Pointer { type_id: _ } => unsafe {
//                     let llvm_val = llvm::core::LLVMBuildAlloca(
//                         builder,
//                         llvm::core::LLVMInt32Type(),
//                         c_id.as_ptr(),
//                     );
//                     llvm::core::LLVMSetAlignment(llvm_val, 4);
//                     llvm_val
//                 },
//                 Type::Array { type_id, size } => unsafe {
//                     let size = {
//                         let llvm_val = const_expression_to_llvm_valueref(
//                             size,
//                             module,
//                             builder,
//                             symbol_table,
//                             &mut Context::new(),
//                         );
//
//                         llvm::core::LLVMConstIntGetSExtValue(llvm_val)
//                     };
//
//                     let array_type = match type_id.as_ref() {
//                         Type::I32 => {
//                             llvm::core::LLVMArrayType(llvm::core::LLVMInt32Type(), size as u32)
//                         }
//                         other => panic!("Array of type: {:?} is not supported!", other),
//                     };
//
//                     let array_size = llvm::core::LLVMConstInt(
//                         llvm::core::LLVMInt32Type(),
//                         size as std::os::raw::c_ulonglong,
//                         0 as std::os::raw::c_int,
//                     );
//
//                     let llvm_alloca = llvm::core::LLVMBuildAlloca(
//                         builder,
//                         array_type,
//                         CStr::from_bytes_with_nul_unchecked(b"array_tmp\0").as_ptr(),
//                     );
//
//                     llvm::core::LLVMSetAlignment(llvm_alloca, 4);
//
//                     llvm_alloca
//                 },
//             };
//
//             symbol_table.insert(id.clone(), l_value);
//
//             let r_value = const_expression_to_llvm_valueref(
//                 expression,
//                 module,
//                 builder,
//                 symbol_table,
//                 &mut Context::new(),
//             );
//
//             match type_id {
//                 Type::Array {
//                     type_id: _,
//                     size: _,
//                 } => unsafe {
//                     let r_value_type = llvm::core::LLVMTypeOf(r_value);
//                     let r_value_type = llvm::core::LLVMGetElementType(r_value_type);
//                     let size = llvm::core::LLVMGetArrayLength(r_value_type);
//                     let array_size = llvm::core::LLVMConstInt(
//                         llvm::core::LLVMInt64Type(),
//                         (size * 4) as std::os::raw::c_ulonglong,
//                         0 as std::os::raw::c_int,
//                     );
//                     llvm::core::LLVMBuildMemCpy(builder, l_value, 4, r_value, 4, array_size);
//                 },
//                 _ => unsafe {
//                     llvm::core::LLVMBuildStore(builder, r_value, l_value);
//                 },
//             }
//         }
//         _ => (),
//     };
// }
//
// struct Context {
//     is_deref_expression: bool,
// }
//
// impl Context {
//     pub fn new() -> Self {
//         Context {
//             is_deref_expression: false,
//         }
//     }
// }
//
// fn const_expression_to_llvm_valueref(
//     expression: &Ast,
//     module: *mut llvm::LLVMModule,
//     builder: *mut llvm::LLVMBuilder,
//     symbol_table: &mut SymbolTable,
//     context: &mut Context,
// ) -> *mut llvm::LLVMValue {
//     match expression {
//         Ast::IntegerLiteral { value } => unsafe {
//             llvm::core::LLVMConstInt(
//                 llvm::core::LLVMInt32Type(),
//                 *value as std::os::raw::c_ulonglong,
//                 0 as std::os::raw::c_int,
//             )
//         },
//         Ast::ArrayLiteral { values } => unsafe {
//             let array_type =
//                 llvm::core::LLVMArrayType(llvm::core::LLVMInt32Type(), values.len() as u32);
//
//             let array_size = llvm::core::LLVMConstInt(
//                 llvm::core::LLVMInt32Type(),
//                 values.len() as std::os::raw::c_ulonglong,
//                 0 as std::os::raw::c_int,
//             );
//
//             let alloc_value = llvm::core::LLVMBuildAlloca(
//                 builder,
//                 array_type,
//                 CStr::from_bytes_with_nul_unchecked(b"array_tmp\0").as_ptr(),
//             );
//
//             llvm::core::LLVMSetAlignment(alloc_value, 4);
//
//             for (index, value) in values.iter().enumerate() {
//                 let llvm_value = const_expression_to_llvm_valueref(
//                     value,
//                     module,
//                     builder,
//                     symbol_table,
//                     context,
//                 );
//
//                 let mut indexes = [
//                     llvm::core::LLVMConstInt(
//                         llvm::core::LLVMInt64Type(),
//                         0,
//                         0,
//                     ),
//                     llvm::core::LLVMConstInt(
//                         llvm::core::LLVMInt64Type(),
//                         index as std::os::raw::c_ulonglong,
//                         0,
//                     ),
//                 ];
//
//                 let ptr = llvm::core::LLVMBuildInBoundsGEP(
//                     builder,
//                     alloc_value,
//                     indexes.as_mut_ptr(),
//                     indexes.len() as u32,
//                     CStr::from_bytes_with_nul_unchecked(b"gep_tmp\0").as_ptr(),
//                 );
//                 let store = llvm::core::LLVMBuildStore(builder, llvm_value, ptr);
//                 llvm::core::LLVMSetAlignment(store, 4);
//             }
//
//             alloc_value
//         },
//         Ast::Identifier { id } => unsafe {
//             if context.is_deref_expression {
//                 symbol_table[id]
//             } else {
//                 match llvm::core::LLVMGetValueKind(symbol_table[id]) {
//                     llvm::LLVMValueKind::LLVMArgumentValueKind => symbol_table[id],
//                     _ => {
//                         match llvm::core::LLVMGetTypeKind(llvm::core::LLVMTypeOf(symbol_table[id]))
//                         {
//                             llvm::LLVMTypeKind::LLVMPointerTypeKind => {
//                                 let llvm_value = symbol_table[id];
//
//                                 let mut indexes = [llvm::core::LLVMConstInt(
//                                     llvm::core::LLVMInt32Type(),
//                                     0 as std::os::raw::c_ulonglong,
//                                     0,
//                                 )];
//
//                                 let llvm_value = llvm::core::LLVMBuildInBoundsGEP(
//                                     builder,
//                                     llvm_value,
//                                     indexes.as_mut_ptr(),
//                                     indexes.len() as u32,
//                                     CStr::from_bytes_with_nul_unchecked(b"gep_tmp\0").as_ptr(),
//                                 );
//
//                                 let det_type = llvm::core::LLVMPointerType(
//                                     llvm::core::LLVMInt8Type(),
//                                     llvm::core::LLVMGetPointerAddressSpace(llvm::core::LLVMTypeOf(
//                                         llvm_value,
//                                     )),
//                                 );
//
//                                 let llvm_value = llvm::core::LLVMBuildBitCast(
//                                     builder,
//                                     llvm_value,
//                                     det_type,
//                                     CStr::from_bytes_with_nul_unchecked(b"bit_cast_tmp\0").as_ptr(),
//                                 );
//
//                                 llvm_value
//                             }
//                             _ => {
//                                 let c_id = CString::new(id.as_str()).unwrap();
//                                 llvm::core::LLVMBuildLoad(builder, symbol_table[id], c_id.as_ptr())
//                             }
//                         }
//                     }
//                 }
//             }
//         },
//         Ast::UnaryExpression {
//             operator,
//             expression,
//         } => match operator {
//             Operator::Neg => unsafe {
//                 llvm::core::LLVMBuildNeg(
//                     builder,
//                     const_expression_to_llvm_valueref(
//                         expression,
//                         module,
//                         builder,
//                         symbol_table,
//                         context,
//                     ),
//                     CStr::from_bytes_with_nul_unchecked(b"neg_tmp\0").as_ptr(),
//                 )
//             },
//             Operator::Deref => {
//                 let mut context = Context::new();
//                 context.is_deref_expression = true;
//                 const_expression_to_llvm_valueref(
//                     expression,
//                     module,
//                     builder,
//                     symbol_table,
//                     &mut context,
//                 )
//             }
//             _ => panic!("{:?} is not a unary operator", operator),
//         },
//         Ast::BinaryExpression {
//             operator,
//             left,
//             right,
//         } => match operator {
//             Operator::Plus => unsafe {
//                 llvm::core::LLVMBuildAdd(
//                     builder,
//                     const_expression_to_llvm_valueref(left, module, builder, symbol_table, context),
//                     const_expression_to_llvm_valueref(
//                         right,
//                         module,
//                         builder,
//                         symbol_table,
//                         context,
//                     ),
//                     CStr::from_bytes_with_nul_unchecked(b"add_tmp\0").as_ptr(),
//                 )
//             },
//             Operator::Minus => unsafe {
//                 llvm::core::LLVMBuildSub(
//                     builder,
//                     const_expression_to_llvm_valueref(left, module, builder, symbol_table, context),
//                     const_expression_to_llvm_valueref(
//                         right,
//                         module,
//                         builder,
//                         symbol_table,
//                         context,
//                     ),
//                     CStr::from_bytes_with_nul_unchecked(b"sub_tmp\0").as_ptr(),
//                 )
//             },
//             Operator::Mul => unsafe {
//                 llvm::core::LLVMBuildMul(
//                     builder,
//                     const_expression_to_llvm_valueref(left, module, builder, symbol_table, context),
//                     const_expression_to_llvm_valueref(
//                         right,
//                         module,
//                         builder,
//                         symbol_table,
//                         context,
//                     ),
//                     CStr::from_bytes_with_nul_unchecked(b"mul_tmp\0").as_ptr(),
//                 )
//             },
//             Operator::Div => unsafe {
//                 llvm::core::LLVMBuildSDiv(
//                     builder,
//                     const_expression_to_llvm_valueref(left, module, builder, symbol_table, context),
//                     const_expression_to_llvm_valueref(
//                         right,
//                         module,
//                         builder,
//                         symbol_table,
//                         context,
//                     ),
//                     CStr::from_bytes_with_nul_unchecked(b"div_tmp\0").as_ptr(),
//                 )
//             },
//             _ => panic!("{:?} is not a binary operator.", operator),
//         },
//         Ast::BlockExpression {
//             statements,
//             return_expression,
//         } => {
//             for statement in statements {
//                 ast_to_llvm_module(statement, module, builder, symbol_table);
//             }
//             const_expression_to_llvm_valueref(
//                 return_expression,
//                 module,
//                 builder,
//                 symbol_table,
//                 context,
//             )
//         }
//         Ast::CallExpression { id, arguments } => unsafe {
//             let mut arguments: Vec<_> = arguments
//                 .iter()
//                 .map(|arg| {
//                     const_expression_to_llvm_valueref(arg, module, builder, symbol_table, context)
//                 })
//                 .collect();
//             llvm::core::LLVMBuildCall(
//                 builder,
//                 symbol_table[id],
//                 arguments.as_mut_ptr(),
//                 arguments.len() as u32,
//                 CStr::from_bytes_with_nul_unchecked(b"call_tmp\0").as_ptr(),
//             )
//         },
//         _ => panic!(),
//     }
// }
