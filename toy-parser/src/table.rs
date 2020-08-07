// use crate::ast::{Ast, Operator, Type as AstType};
// use std::collections::HashMap;
// use std::rc::Rc;
//
// pub type Identifier = String;
//
// #[derive(Clone, Debug)]
// pub enum Type {
//     I32,
//     Pointer {
//         type_: Box<Type>,
//     },
//     Array {
//         type_: Box<Type>,
//         size: usize,
//     },
//     Function {
//         parameters: Vec<Type>,
//         return_type: Box<Type>,
//     },
// }
//
// impl From<&AstType> for Type {
//     fn from(ast: &AstType) -> Self {
//         Type::I32
//     }
// }
//
// #[derive(Debug)]
// pub enum Data {
//     Type(Type),
//     Function { type_: Type, parameters: Table },
//     Module(Table),
// }
//
// #[derive(Debug)]
// pub enum TypedAst {
//     Module {
//         contents: Vec<TypedAst>,
//     },
//     FunctionDeclaration {
//         id: String,
//         parameters: Vec<(String, Type)>,
//     },
//     FunctionDefinition {
//         id: String,
//         parameters: Vec<(String, Type)>,
//         body: Box<TypedAst>,
//     },
//     VariableDefinition {
//         id: String,
//         type_: Type,
//         expression: Box<TypedAst>,
//     },
//     BlockExpression {
//         statements: Vec<TypedAst>,
//         return_expression: Box<TypedAst>,
//     },
//     UnaryExpression {
//         operator: Operator,
//         expression: Box<TypedAst>,
//         result_type: Type,
//     },
//     BinaryExpression {
//         operator: Operator,
//         left: Box<TypedAst>,
//         right: Box<TypedAst>,
//         result_type: Type,
//     },
//     CallExpression {
//         id: String,
//         arguments: Vec<TypedAst>,
//         result_type: Type,
//     },
//     AccessExpression {
//         base: Box<TypedAst>,
//         offset: Box<TypedAst>,
//         result_type: Type,
//     },
//     IntegerLiteral {
//         value: i32,
//     },
//     ArrayLiteral {
//         values: Vec<TypedAst>,
//         result_type: Type,
//     },
//     Identifier {
//         id: String,
//     },
//     None,
// }
//
// #[derive(Debug)]
// pub struct Table {
//     pub context: HashMap<Identifier, Data>,
//     pub super_context: Option<Rc<Table>>,
// }
//
// impl Table {
//     pub fn new() -> Self {
//         Table {
//             context: HashMap::new(),
//             super_context: None,
//         }
//     }
//
//     pub fn from_ast(ast: &Ast) -> Self {
//         let mut root = Table::new();
//
//         root.make_context(ast);
//
//         root
//     }
//
//     pub fn put(father: Rc<Table>) -> Self {
//         Table {
//             context: HashMap::new(),
//             super_context: Some(father),
//         }
//     }
//
//     pub fn get(&self, id: &Identifier) -> Option<&Data> {
//         self.context.get(id)
//     }
//
//     pub fn make_context(&mut self, ast: &Ast) {
//         use Ast::*;
//         match ast {
//             Module { contents } => {
//                 let module_name = "root".to_string();
//
//                 let mut module_table = Table::new();
//
//                 for content in contents {
//                     module_table.make_context(content);
//                 }
//
//                 self.context.insert(module_name, Data::Module(module_table));
//             }
//             VariableDefinition {
//                 id,
//                 type_id: _,
//                 expression: _,
//             } => {
//                 self.context.insert(id.clone(), Data::Type(Type::I32));
//             }
//             FunctionDeclaration { id, parameters } => {
//                 let function_type = Type::Function {
//                     parameters: vec![Type::I32; parameters.len()],
//                     return_type: Box::new(Type::I32),
//                 };
//
//                 self.context.insert(id.clone(), Data::Type(function_type));
//             }
//             FunctionDefinition {
//                 id,
//                 parameters,
//                 body,
//             } => {
//                 let function_type = Type::Function {
//                     parameters: vec![Type::I32; parameters.len()],
//                     return_type: Box::new(Type::I32),
//                 };
//
//                 let mut parameters_table = Table::new();
//
//                 for (id, _type) in parameters {
//                     parameters_table
//                         .context
//                         .insert(id.clone(), Data::Type(Type::I32));
//                 }
//
//                 self.context.insert(
//                     id.clone(),
//                     Data::Function {
//                         type_: function_type,
//                         parameters: parameters_table,
//                     },
//                 );
//             }
//             BlockExpression {
//                 statements,
//                 return_expression: _,
//             } => {
//                 for statement in statements {
//                     self.make_context(statement);
//                 }
//             }
//             _ => (),
//         }
//     }
//
//     pub fn build_typed_ast(&self, ast: &Ast) -> TypedAst {
//         use Ast::*;
//         match ast {
//             Module { contents } => {
//                 let mut typed_contents = Vec::new();
//                 for statement in contents {
//                     typed_contents.push(self.build_typed_ast(statement));
//                 }
//
//                 TypedAst::Module {
//                     contents: typed_contents,
//                 }
//             }
//             FunctionDeclaration { id, parameters } => TypedAst::FunctionDeclaration {
//                 id: id.clone(),
//                 parameters: parameters
//                     .iter()
//                     .map(|(id, type_)| (id.clone(), Type::from(type_)))
//                     .collect(),
//             },
//             FunctionDefinition {
//                 id,
//                 parameters,
//                 body,
//             } => TypedAst::FunctionDefinition {
//                 id: id.clone(),
//                 parameters: parameters
//                     .iter()
//                     .map(|(id, type_)| (id.clone(), Type::from(type_)))
//                     .collect(),
//                 body: Box::new(self.build_typed_ast(body)),
//             },
//             VariableDefinition {
//                 id,
//                 type_id,
//                 expression,
//             } => TypedAst::VariableDefinition {
//                 id: id.clone(),
//                 type_: Type::from(type_id),
//                 expression: Box::new(self.build_typed_ast(expression)),
//             },
//             BlockExpression {
//                 statements,
//                 return_expression,
//             } => TypedAst::BlockExpression {
//                 statements: statements
//                     .iter()
//                     .map(|statement| self.build_typed_ast(statement))
//                     .collect(),
//                 return_expression: Box::new(self.build_typed_ast(return_expression)),
//             },
//             UnaryExpression {
//                 operator,
//                 expression,
//             } => match operator {
//                 Operator::Neg => TypedAst::UnaryExpression {
//                     operator: *operator,
//                     expression: Box::new(self.build_typed_ast(expression)),
//                     result_type: Type::I32,
//                 },
//                 Operator::Deref | _ => TypedAst::None,
//             },
//             BinaryExpression {
//                 operator,
//                 left,
//                 right,
//             } => match operator {
//                 Operator::Mul | Operator::Div | Operator::Plus | Operator::Minus => {
//                     TypedAst::BinaryExpression {
//                         operator: *operator,
//                         left: Box::new(self.build_typed_ast(left)),
//                         right: Box::new(self.build_typed_ast(right)),
//                         result_type: Type::I32,
//                     }
//                 }
//                 _ => TypedAst::None,
//             },
//             CallExpression { id, arguments } => TypedAst::CallExpression {
//                 id: id.clone(),
//                 arguments: arguments
//                     .iter()
//                     .map(|ast| self.build_typed_ast(ast))
//                     .collect(),
//                 result_type: match self.get(id).unwrap() {
//                     Data::Function {
//                         type_,
//                         parameters: _,
//                     } => type_.clone(),
//                     _ => panic!(),
//                 },
//             },
//             AccessExpression { base, offset } => TypedAst::AccessExpression {
//                 base: Box::new(self.build_typed_ast(base)),
//                 offset: Box::new(self.build_typed_ast(offset)),
//                 result_type: Type::I32,
//             },
//             IntegerLiteral { value } => TypedAst::IntegerLiteral { value: *value },
//             ArrayLiteral { values } => TypedAst::ArrayLiteral {
//                 result_type: Type::I32,
//                 values: values
//                     .iter()
//                     .map(|value| self.build_typed_ast(value))
//                     .collect(),
//             },
//             Ast::Identifier { id } => TypedAst::Identifier { id: id.clone() },
//             None => TypedAst::None,
//         }
//     }
//
//     fn type_check(&self, ast: &Ast) {
//         use Ast::*;
//
//         match ast {
//             Module { contents } => {
//                 for content in contents {
//                     self.type_check(content);
//                 }
//             }
//             Ast::FunctionDeclaration {
//                 id: _,
//                 parameters: _,
//             } => (),
//             Ast::FunctionDefinition {
//                 id,
//                 parameters: _,
//                 body,
//             } => {
//                 let context = self.get(&id).unwrap();
//                 match context {
//                     Data::Function { type_, parameters } => {
//                         parameters.type_check(body);
//                     }
//                     _ => panic!("Function not Function"),
//                 }
//             }
//             BlockExpression {
//                 statements,
//                 return_expression,
//             } => {
//                 for statement in statements {
//                     self.type_check(statement);
//                 }
//
//                 self.type_check(return_expression);
//             }
//             UnaryExpression {
//                 operator: _,
//                 expression,
//             } => {
//                 self.type_check(expression);
//             }
//             BinaryExpression {
//                 operator: _,
//                 left,
//                 right,
//             } => {
//                 panic!("binary operation mismatch types");
//             }
//             other => panic!("{:?} ast node not supported by typechecker", other),
//         }
//     }
// }
