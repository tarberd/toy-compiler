use crate::ast::*;
use crate::visitor::{AstVisitor, Visitable};

struct TypeChecker {
}

impl TypeChecker {
    fn type_of(&self, _id: &Identifier) -> Type {
        Type::I32
    }
}

impl AstVisitor<(), Type> for TypeChecker {
    type Return = Type;
    type Environment = ();

    fn visit_module_statement(
        &mut self,
        env: Self::Environment,
        m: &ModuleStatement,
    ) -> Self::Return {
        for statement in &m.statements {
            statement.accept(env, self);
        }
        Type::None
    }

    fn visit_statement(&mut self, env: Self::Environment, s: &Statement) -> Self::Return {
        use Statement::*;
        match s {
            Module(m) => m.accept(env, self),
            ExternFunctionDeclaration(s) => s.accept(env, self),
            FunctionDefinition(s) => s.accept(env, self),
            VariableDefinition(s) => s.accept(env, self),
        }
    }

    fn visit_extern_function_declaration_statement(
        &mut self,
        env: Self::Environment,
        _f: &ExternFunctionDeclarationStatement,
    ) -> Self::Return {
        Type::None
    }

    fn visit_function_definition_statement(
        &mut self,
        env: Self::Environment,
        function: &FunctionDefinitionStatement,
    ) -> Self::Return {
        let body_return_type = function.body.accept(env, self);
        if function.return_type != body_return_type {
            panic!(
                "Functintion body's type: {:?} differs from function return type: {:?}",
                body_return_type, function.return_type
            );
        }
        Type::None
    }

    fn visit_variable_definition_statement(
        &mut self,
        env: Self::Environment,
        variable: &VariableDefinitionStatement,
    ) -> Self::Return {
        variable.accept(env, self)
    }

    fn visit_expression(
        &mut self,
        env: Self::Environment,
        expression: &Expression,
    ) -> Self::Return {
        use Expression::*;
        match expression {
            Block(expr) => expr.accept(env, self),
            Unary(expr) => expr.accept(env, self),
            Binary(expr) => expr.accept(env, self),
            Call(expr) => expr.accept(env, self),
            Access(expr) => expr.accept(env, self),
            Array(expr) => expr.accept(env, self),
            Integer(expr) => expr.accept(env, self),
            Identifier(expr) => expr.accept(env, self),
        }
    }

    fn visit_block_expression(
        &mut self,
        env: Self::Environment,
        block: &BlockExpression,
    ) -> Self::Return {
        match &block.return_expression {
            Some(expr) => expr.accept(env, self),
            None => Type::Void,
        }
    }

    fn visit_unary_expression(
        &mut self,
        env: Self::Environment,
        unary: &UnaryExpression,
    ) -> Self::Return {
        unary.expression.accept(env, self)
    }

    fn visit_binary_expression(
        &mut self,
        env: Self::Environment,
        binary: &BinaryExpression,
    ) -> Self::Return {
        match (
            binary.left.accept(env, self),
            binary.right.accept(env, self),
        ) {
            (left, right) => {
                if left == right {
                    right
                } else {
                    panic!("binary expression difers in type");
                }
            }
        }
    }

    fn visit_call_expression(
        &mut self,
        env: Self::Environment,
        call: &CallExpression,
    ) -> Self::Return {
        call.callee.accept(env, self)
    }

    fn visit_access_expression(
        &mut self,
        env: Self::Environment,
        access: &AccessExpression,
    ) -> Self::Return {
        let base_type = access.base.accept(env, self);

        match base_type {
            Type::Array { type_id, size: _ } => *type_id,
            _ => panic!("access type error"),
        }
    }

    fn visit_array_literal(
        &mut self,
        env: Self::Environment,
        array: &ArrayLiteral,
    ) -> Self::Return {
        let expr_types: Vec<Type> = array
            .initialize_expressions
            .iter()
            .map(|expression| expression.accept(env, self))
            .collect();

        match expr_types.first() {
            Some(nested_type) => {
                for expr_type in expr_types.iter().next() {
                    if nested_type != expr_type {
                        panic!("expression type difer in array init expression");
                    }
                }

                Type::Array {
                    type_id: Box::new(nested_type.clone()),
                    size: Expression::Integer(IntegerLiteral { value: 0 }),
                }
            }
            None => Type::Array {
                type_id: Box::new(Type::None),
                size: Expression::Integer(IntegerLiteral { value: 0 }),
            },
        }
    }

    fn visit_integer_literal(
        &mut self,
        env: Self::Environment,
        _: &IntegerLiteral,
    ) -> Self::Return {
        Type::IntegerLiteral
    }

    fn visit_identifier(&mut self, env: Self::Environment, id: &Identifier) -> Self::Return {
        self.type_of(id)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn typecheck() {
        use crate::parser::ModuleParser;
        let code = "
        fn sum(lhs: i32, rhs: i32): i32  => lhs + rhs;
        fn square(lhs: i32, rhs: i32): i32 => { lhs * rhs };
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut type_checker = super::TypeChecker {};

        use super::AstVisitor;

        type_checker.visit_module_statement((), &module);
    }
}
