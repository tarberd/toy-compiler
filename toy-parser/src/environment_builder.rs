use crate::ast::*;
use crate::visitor::{AstVisitor, Visitable};

use std::collections::HashMap;

#[derive(Debug)]
struct Environment {
    table: HashMap<Identifier, Type>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            table: HashMap::new(),
        }
    }

    fn put(&self) -> Self {
        todo!()
    }

    fn insert(&mut self, id: Identifier, type_: Type) {
        self.table.insert(id, type_);
    }

    fn get(&self, id: &Identifier) -> Option<&Type> {
        self.table.get(id)
    }
}

struct EnvironmentBuilder {}

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
        todo!()
    }

    fn visit_function_definition_statement(
        &mut self,
        env: Self::Environment,
        function: &FunctionDefinitionStatement,
    ) -> Self::Return {
        todo!()
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

    fn visit_expression(
        &mut self,
        env: Self::Environment,
        expression: &Expression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_block_expression(
        &mut self,
        env: Self::Environment,
        block: &BlockExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_unary_expression(
        &mut self,
        env: Self::Environment,
        unary: &UnaryExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_binary_expression(
        &mut self,
        env: Self::Environment,
        binary: &BinaryExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_call_expression(
        &mut self,
        env: Self::Environment,
        call: &CallExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_access_expression(
        &mut self,
        env: Self::Environment,
        access: &AccessExpression,
    ) -> Self::Return {
        todo!()
    }
    fn visit_array_literal(
        &mut self,
        env: Self::Environment,
        array: &ArrayLiteral,
    ) -> Self::Return {
        todo!()
    }
    fn visit_integer_literal(
        &mut self,
        env: Self::Environment,
        literal: &IntegerLiteral,
    ) -> Self::Return {
        todo!()
    }
    fn visit_identifier(&mut self, env: Self::Environment, id: &Identifier) -> Self::Return {
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
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = super::EnvironmentBuilder {};

        use super::AstVisitor;

        let env = env_builder.visit_module_statement(super::Environment::new(), &module);

        assert_eq!(env.get(&Identifier::from("x".to_string())), Some(&Type::I32));
    }
}
