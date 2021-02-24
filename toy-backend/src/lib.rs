use llvm_sys as llvm;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use toy_lexer::Span;
use toy_parser2::ast::{Expression, ExpressionKind, Function, RootModule};

pub struct Backend {
    target_machine: *mut llvm::target_machine::LLVMOpaqueTargetMachine,
}

impl Backend {
    pub fn new() -> Self {
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

        Self { target_machine }
    }

    pub fn emit_llvm(&self, module: *mut llvm::LLVMModule) {
        let codegen = llvm::target_machine::LLVMCodeGenFileType::LLVMObjectFile;

        let err = std::ptr::null_mut();

        if let 1 = unsafe {
            llvm::target_machine::LLVMTargetMachineEmitToFile(
                self.target_machine,
                module,
                b"out.o\0".as_ptr() as *mut std::os::raw::c_char,
                codegen,
                err,
            )
        } {
            unsafe { llvm::core::LLVMDisposeMessage(*err) };
            panic!("Failed to initialize llvm native target");
        };
    }
}

pub fn print_module(module: *mut llvm::LLVMModule) {
    let asm = unsafe { std::ffi::CStr::from_ptr(llvm::core::LLVMPrintModuleToString(module)) };

    println!("{}", asm.to_str().expect("c string error on output"));
}

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
pub struct Lower<'input> {
    llvm_module: *mut llvm::LLVMModule,
    llvm_builder: *mut llvm::LLVMBuilder,
    source: &'input str,
}

impl<'input> Lower<'input> {
    pub fn new(source: &'input str, module_name: String) -> Self {
        let llvm_context = unsafe { llvm::core::LLVMContextCreate() };

        let llvm_module = unsafe {
            let c_module_name = CString::new(module_name).unwrap();

            llvm::core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), llvm_context)
        };

        let llvm_builder = unsafe { llvm::core::LLVMCreateBuilder() };

        Self {
            llvm_module,
            llvm_builder,
            source,
        }
    }

    pub fn lower_root_module(&mut self, module: &RootModule) -> *mut llvm::LLVMModule {
        let mut records = self.activate_functions(module);
        for function in module.functions.iter() {
            self.lower_function(&mut records, function);
        }
        self.llvm_module
    }

    fn activate_functions(&self, module: &RootModule) -> ActivationRecords {
        let mut activation_records = ActivationRecords::new();

        for function in module.functions.iter() {
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
                let c_name = CString::new(self.span_to_str(&function.id.span)).unwrap();

                llvm::core::LLVMAddFunction(self.llvm_module, c_name.as_ptr(), function_type)
            };

            for (index, param) in function.parameters.iter().enumerate() {
                unsafe {
                    let llvm_param = llvm::core::LLVMGetParam(llvm_function, index as u32);
                    let c_name = CString::new(self.span_to_str(&param.id.span)).unwrap();
                    let c_name_len = c_name.as_bytes().len();
                    llvm::core::LLVMSetValueName2(llvm_param, c_name.as_ptr(), c_name_len);
                };
            }

            activation_records.insert(
                self.span_to_str(&function.id.span).to_string(),
                llvm_function,
            );
        }

        activation_records
    }

    fn lower_function(&mut self, records: &mut ActivationRecords, function: &Function) {
        let llvm_func = records[&self.span_to_str(&function.id.span).to_string()];

        unsafe {
            let block = llvm::core::LLVMAppendBasicBlock(
                llvm_func,
                b"entry\0".as_ptr() as *const std::os::raw::c_char,
            );
            llvm::core::LLVMPositionBuilderAtEnd(self.llvm_builder, block);
            llvm::core::LLVMBuildRet(
                self.llvm_builder,
                self.lower_expression(&function.body.expression),
            );
        };
    }

    fn lower_expression(&mut self, expr: &Expression) -> *mut llvm::LLVMValue {
        match &expr.kind {
            ExpressionKind::Block(_) => {
                todo!()
            }
            ExpressionKind::Binary(lhs, op, rhs) => {
                let lhs_val = self.lower_expression(lhs);
                let rhs_val = self.lower_expression(rhs);

                use toy_parser2::ast::BinaryOperator;
                match op {
                    BinaryOperator::Sum => unsafe {
                        llvm::core::LLVMBuildAdd(
                            self.llvm_builder,
                            lhs_val,
                            rhs_val,
                            b"add_tmp\0".as_ptr() as *const c_char,
                        )
                    },
                }
            }
            ExpressionKind::Literal(lit) => unsafe {
                let literal: String = self
                    .span_to_str(&lit.literal)
                    .chars()
                    .filter(|c| *c != '_')
                    .collect();
                let value: usize = literal.parse().unwrap();

                llvm::core::LLVMConstInt(
                    llvm::core::LLVMInt32Type(),
                    value as std::os::raw::c_ulonglong,
                    0 as std::os::raw::c_int,
                )
            },
        }
    }

    fn span_to_str(&self, span: &Span) -> &str {
        &self.source[span.offset..span.offset + span.len]
    }
}
