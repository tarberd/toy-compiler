use llvm_sys as llvm;
use std::collections::HashMap;
use std::ffi::CString;
use toy_parser::ast::{IntType, Literal, LiteralIntType, UIntType};
use toy_parser::typecheck::{Block, Module, TypedExpression};

pub struct ActivationRecords {
    table_stack: Vec<HashMap<String, *mut llvm::LLVMValue>>,
}

impl ActivationRecords {
    fn new() -> Self {
        Self {
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

impl std::ops::Index<&String> for ActivationRecords {
    type Output = *mut llvm::LLVMValue;

    fn index(&self, string: &String) -> &Self::Output {
        self.table_stack
            .iter()
            .rev()
            .find_map(|table| table.get(string))
            .unwrap()
    }
}

pub fn populate_llvm_module(llvm_module: *mut llvm::LLVMModule, src_module: Module) -> () {
    let builder = unsafe { llvm::core::LLVMCreateBuilder() };

    let mut activation_records = ActivationRecords::new();

    for function in &src_module.functions {
        let mut parameter_types =
            unsafe { vec![llvm::core::LLVMInt32Type(); function.parameters.len()] };

        let function_type = unsafe {
            llvm::core::LLVMFunctionType(
                llvm::core::LLVMInt32Type(),
                parameter_types.as_mut_ptr() as *mut *mut llvm::LLVMType,
                parameter_types.len() as std::os::raw::c_uint,
                0,
            )
        };

        let llvm_function = unsafe {
            let c_name = CString::new(function.id.value.as_str()).unwrap();

            llvm::core::LLVMAddFunction(llvm_module, c_name.as_ptr(), function_type)
        };

        activation_records.insert(function.id.value.clone(), llvm_function);
    }

    for function in &src_module.functions {
        let llvm_function = activation_records[&function.id.value];

        let llvm_function_params = unsafe {
            let llvm_function_params = Vec::with_capacity(function.parameters.len());
            let mut llvm_function_params = std::mem::ManuallyDrop::new(llvm_function_params);

            llvm::core::LLVMGetParams(llvm_function, llvm_function_params.as_mut_ptr());

            Vec::from_raw_parts(
                llvm_function_params.as_mut_ptr(),
                llvm_function_params.capacity(),
                llvm_function_params.capacity(),
            )
        };

        activation_records.push();

        for (value, name) in llvm_function_params
            .iter()
            .zip(function.parameters.iter().map(|x| x.0.value.clone()))
        {
            println!("value name parameter: {}", name);
            activation_records.insert(name.clone(), *value);

            let c_name = CString::new(name.as_str()).unwrap();
            let c_name_len = c_name.as_bytes().len();

            unsafe { llvm::core::LLVMSetValueName2(*value, c_name.as_ptr(), c_name_len) }
        }

        let function_block = unsafe {
            llvm::core::LLVMAppendBasicBlock(
                llvm_function,
                b"body\0".as_ptr() as *const std::os::raw::c_char,
            )
        };

        unsafe {
            llvm::core::LLVMPositionBuilderAtEnd(builder, function_block);
            llvm::core::LLVMBuildRet(
                builder,
                build_llvm_expression(llvm_module, builder, &function.body),
            );
        };

        activation_records.pop();
    }

    unsafe {
        llvm::core::LLVMDisposeBuilder(builder);
    };
}

fn build_llvm_expression(
    llvm_module: *mut llvm::LLVMModule,
    llvm_builder: *mut llvm::LLVMBuilder,
    expr: &TypedExpression,
) -> *mut llvm::LLVMValue {
    match expr {
        TypedExpression::Block(block) => build_llvm_block(llvm_module, llvm_builder, block),
        TypedExpression::Literal(literal) => build_llvm_literal(llvm_module, llvm_builder, literal),
        _ => todo!(),
    }
}

fn build_llvm_block(
    llvm_module: *mut llvm::LLVMModule,
    llvm_builder: *mut llvm::LLVMBuilder,
    block: &Block,
) -> *mut llvm::LLVMValue {
    build_llvm_expression(llvm_module, llvm_builder, &block.return_expression)
}

fn build_llvm_literal(
    llvm_module: *mut llvm::LLVMModule,
    llvm_builder: *mut llvm::LLVMBuilder,
    literal: &Literal,
) -> *mut llvm::LLVMValue {
    match literal {
        Literal::Int(value, ty) => match ty {
            LiteralIntType::Signed(ty) => match ty {
                IntType::I32 => unsafe {
                    llvm::core::LLVMConstInt(
                        llvm::core::LLVMInt32Type(),
                        *value as std::os::raw::c_ulonglong,
                        0 as std::os::raw::c_int,
                    )
                },
                _ => todo!(),
            },
            LiteralIntType::Unsigned(ty) => todo!(),
            LiteralIntType::Unsufixed => todo!(),
        },
        _ => todo!(),
    }
}
