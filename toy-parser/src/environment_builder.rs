use crate::ast::*;
use crate::visitor::{AstVisitor, Visitable};

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    table: HashMap<Identifier, Type>,
    father: Option<Rc<Self>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            table: HashMap::new(),
            father: None,
        }
    }

    pub fn put(father: Rc<Self>) -> Self {
        Environment {
            table: HashMap::new(),
            father: Some(father),
        }
    }

    pub fn insert(&mut self, id: Identifier, type_: Type) {
        self.table.insert(id, type_);
    }

    pub fn get(&self, id: &Identifier) -> Option<&Type> {
        match self.table.get(id) {
            Some(t) => Some(t),
            None => match &self.father {
                Some(f) => f.get(id),
                None => None,
            },
        }
    }
}

pub struct EnvironmentBuilder {}

impl AstVisitor<Environment, Environment> for EnvironmentBuilder {
    type Return = Environment;
    type Environment = Environment;

    fn visit_statement(&mut self, env: Self::Environment, s: &Statement) -> Self::Return {
        use Statement::*;
        match s {
            Module(m) => m.accept(env, self),
            ExternFunctionDeclaration(s) => s.accept(env, self),
            FunctionDefinition(s) => s.accept(env, self),
            VariableDefinition(s) => s.accept(env, self),
            Return(s) => s.accept(env, self),
        }
    }

    fn visit_module_statement(
        &mut self,
        env: Self::Environment,
        m: &ModuleStatement,
    ) -> Self::Return {
        let mut env = env;

        for statement in &m.statements {
            env = statement.accept(env, self);
        }

        env
    }

    fn visit_extern_function_declaration_statement(
        &mut self,
        env: Self::Environment,
        f: &ExternFunctionDeclarationStatement,
    ) -> Self::Return {
        let mut env = env;
        env.insert(
            f.id.clone(),
            Type::Function {
                parameters: f
                    .parameters
                    .iter()
                    .map(|(_id, type_)| type_.clone())
                    .collect(),
                return_type: Box::new(f.return_type.clone()),
            },
        );
        env
    }

    fn visit_function_definition_statement(
        &mut self,
        env: Self::Environment,
        function: &FunctionDefinitionStatement,
    ) -> Self::Return {
        let mut env = env;
        env.insert(
            function.id.clone(),
            Type::Function {
                parameters: function
                    .parameters
                    .iter()
                    .map(|(_id, type_)| type_.clone())
                    .collect(),
                return_type: Box::new(function.return_type.clone()),
            },
        );
        env
    }

    fn visit_variable_definition_statement(
        &mut self,
        env: Self::Environment,
        variable: &VariableDefinitionStatement,
    ) -> Self::Return {
        let mut env = env;
        env.insert(variable.id.clone(), variable.type_.clone());
        env
    }

    fn visit_return_statement(&mut self, env: Environment, return_statement: &ReturnStatement) -> Self::Return {
        env
    }

    fn visit_expression(
        &mut self,
        _env: Self::Environment,
        _expression: &Expression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_block_expression(
        &mut self,
        _env: Self::Environment,
        _block: &BlockExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_unary_expression(
        &mut self,
        _env: Self::Environment,
        _unary: &UnaryExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_binary_expression(
        &mut self,
        _env: Self::Environment,
        _binary: &BinaryExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_call_expression(
        &mut self,
        _env: Self::Environment,
        _call: &CallExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_access_expression(
        &mut self,
        _env: Self::Environment,
        _access: &AccessExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_array_literal(
        &mut self,
        _env: Self::Environment,
        _array: &ArrayLiteral,
    ) -> Self::Return {
        todo!()
    }
    fn visit_integer_literal(
        &mut self,
        _env: Self::Environment,
        _literal: &IntegerLiteral,
    ) -> Self::Return {
        todo!()
    }
    fn visit_boolean_literal(&mut self, _env: Environment, _boolean: &BooleanLiteral) -> Environment {
        todo!()
    }
    fn visit_identifier(&mut self, _env: Self::Environment, _id: &Identifier) -> Self::Return {
        todo!()
    }

    fn visit_if_expression(&mut self, _env: Environment, _if_expression: &IfExpression) -> Environment {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_env_build() {
        use super::*;

        use crate::parser::ModuleParser;
        let code = "
        let x : i32 = 5;

        extern fn putc();

        extern fn extern_sum(lhs: i32, rhs: i32) : i32;

        fn nop() => {};

        fn sum(lhs: i32, rhs: i32) => lhs + rhs;
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = super::EnvironmentBuilder {};

        use super::AstVisitor;

        let env = env_builder.visit_module_statement(super::Environment::new(), &module);

        assert_eq!(
            env.get(&Identifier::from("x".to_string())),
            Some(&Type::I32)
        );

        assert_eq!(
            env.get(&Identifier::from("nop".to_string())),
            Some(&Type::Function {
                parameters: vec![],
                return_type: Box::new(Type::Void),
            })
        );

        assert_eq!(
            env.get(&Identifier::from("putc".to_string())),
            Some(&Type::Function {
                parameters: vec![],
                return_type: Box::new(Type::Void),
            })
        );

        assert_eq!(
            env.get(&Identifier::from("extern_sum".to_string())),
            Some(&Type::Function {
                parameters: vec![Type::I32, Type::I32],
                return_type: Box::new(Type::I32),
            })
        );
    }
}
