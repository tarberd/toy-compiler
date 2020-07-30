use crate::ast::Ast;
use std::collections::HashMap;

pub type Identifier = String;

#[derive(Clone, Debug)]
pub enum Type {
    I32,
    Pointer {
        type_: Box<Type>,
    },
    Array {
        type_: Box<Type>,
        size: usize,
    },
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[derive(Debug)]
pub enum Data {
    Type(Type),
    Table(Table),
}

#[derive(Debug)]
pub struct Table {
    pub context: HashMap<Identifier, Data>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            context: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from_ast(ast: &Ast) -> Self {
        let mut root = Table::new();

        root.make_context(ast);

        root
    }

    fn make_context(&mut self, ast: &Ast) {
        use Ast::*;
        match ast {
            Module { contents } => {
                let module_name = "root".to_string();

                let mut module_table = Table::new();

                for content in contents {
                    module_table.make_context(content);
                }

                self.context.insert(module_name, Data::Table(module_table));
            }
            FunctionDefinition {
                id,
                parameters,
                body: _,
            } => {
                let function_type = Type::Function {
                    parameters: vec![Type::I32; parameters.len()],
                    return_type: Box::new(Type::I32),
                };

                self.context.insert(id.clone(), Data::Type(function_type));
            }
            _ => (),
        }
    }
}
