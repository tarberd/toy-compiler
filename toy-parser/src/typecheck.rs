use crate::ast::*;
use crate::environment_builder::{Environment, EnvironmentBuilder};
use crate::visitor::{AstVisitor, Visitable};

struct TypeChecker {}

use std::rc::Rc;

impl AstVisitor<Rc<Environment>, Type> for TypeChecker {
    type Return = Type;
    type Environment = Rc<Environment>;

    fn visit_module_statement(
        &mut self,
        env: Self::Environment,
        m: &ModuleStatement,
    ) -> Self::Return {
        for statement in &m.statements {
            statement.accept(Rc::clone(&env), self);
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
            Return(s) => s.accept(env, self),
        }
    }

    fn visit_extern_function_declaration_statement(
        &mut self,
        _env: Self::Environment,
        _f: &ExternFunctionDeclarationStatement,
    ) -> Self::Return {
        Type::None
    }

    fn visit_function_definition_statement(
        &mut self,
        env: Self::Environment,
        function: &FunctionDefinitionStatement,
    ) -> Self::Return {
        let mut env = Environment::put(env);
        for (id, type_) in &function.parameters {
            env.insert(id.clone(), type_.clone())
        }
        env.insert(
            Identifier {
                value: String::from("expected_return_type"),
            },
            function.return_type.clone(),
        );
        let env = Rc::new(env);
        let body_return_type = function.body.accept(env, self);
        if function.return_type != body_return_type {
            panic!(
                concat!(
                    "In function named {}: expected return type {:?}",
                    " differs from function's body return type {:?}"
                ),
                function.id.value, function.return_type, body_return_type,
            );
        }
        Type::None
    }

    fn visit_variable_definition_statement(
        &mut self,
        env: Self::Environment,
        variable: &VariableDefinitionStatement,
    ) -> Self::Return {
        let init_expr_type = variable.initialize_expression.accept(env, self);
        if variable.type_ != init_expr_type {
            panic!(
                "Inititalize expression for {:?} differs in type. Expected: {:?}, Found: {:?}.",
                variable.id, variable.type_, init_expr_type
            );
        }

        Type::None
    }

    fn visit_return_statement(
        &mut self,
        env: Self::Environment,
        return_statement: &ReturnStatement,
    ) -> Self::Return {
        let return_type = return_statement.expression.accept(Rc::clone(&env), self);

        if let Some(function_return_type) = env.get(&Identifier {
            value: String::from("expected_return_type"),
        }) {
            if function_return_type == &return_type {
                Type::None
            } else {
                panic!(
                    "return statement differs in type, expected: {:?}, found: {:?}.",
                    function_return_type, return_type
                );
            }
        } else {
            panic!("return statement is not inside a function");
        }
    }

    fn visit_expression(
        &mut self,
        env: Self::Environment,
        expression: &Expression,
    ) -> Self::Return {
        use Expression::*;
        match expression {
            Block(expr) => expr.accept(env, self),
            If(expr) => expr.accept(env, self),
            Unary(expr) => expr.accept(env, self),
            Binary(expr) => expr.accept(env, self),
            Call(expr) => expr.accept(env, self),
            Access(expr) => expr.accept(env, self),
            Array(expr) => expr.accept(env, self),
            Integer(expr) => expr.accept(env, self),
            Boolean(expr) => expr.accept(env, self),
            Identifier(expr) => expr.accept(env, self),
        }
    }

    fn visit_block_expression(
        &mut self,
        env: Self::Environment,
        block: &BlockExpression,
    ) -> Self::Return {
        let mut env = Rc::clone(&env);

        for statement in &block.statements {
            match statement {
                Statement::FunctionDefinition(_) => {
                    panic!("Function definition inside block is not allowed");
                }
                _ => {
                    let env_not_rc = Environment::put(Rc::clone(&env));
                    env = Rc::new(Environment::put(Rc::new(
                        statement.accept(env_not_rc, &mut EnvironmentBuilder {}),
                    )));
                    statement.accept(Rc::clone(&env), self);
                }
            }
        }

        match &block.return_expression {
            Some(expr) => expr.accept(Rc::clone(&env), self),
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
            binary.left.accept(Rc::clone(&env), self),
            binary.right.accept(env, self),
        ) {
            (left, right) => {
                if left == right {
                    right
                } else {
                    panic!(
                        "Binary expression differs in type. lhs: {:?} rhs: {:?}",
                        left, right
                    );
                }
            }
        }
    }

    fn visit_call_expression(
        &mut self,
        env: Self::Environment,
        call: &CallExpression,
    ) -> Self::Return {
        if let Type::Function {
            parameters,
            return_type,
        } = call.callee.accept(Rc::clone(&env), self)
        {
            if !call
                .arguments
                .iter()
                .map(|argument| argument.accept(Rc::clone(&env), self))
                .eq(parameters.iter().cloned())
            {
                panic!("Function arguments type mismatch.");
            }
            *return_type
        } else {
            panic!("call expression on non function type");
        }
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
            .map(|expression| expression.accept(Rc::clone(&env), self))
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
                    size: Expression::Integer(Box::new(IntegerLiteral {
                        value: 0,
                        type_: Type::IntegerLiteral,
                    })),
                }
            }
            None => Type::Array {
                type_id: Box::new(Type::None),
                size: Expression::Integer(Box::new(IntegerLiteral {
                    value: 0,
                    type_: Type::IntegerLiteral,
                })),
            },
        }
    }

    fn visit_integer_literal(
        &mut self,
        _env: Self::Environment,
        literal: &IntegerLiteral,
    ) -> Self::Return {
        literal.type_.clone()
    }

    fn visit_boolean_literal(&mut self, _env: Rc<Environment>, _boolean: &BooleanLiteral) -> Type {
        Type::Boolean
    }

    fn visit_identifier(&mut self, env: Self::Environment, id: &Identifier) -> Self::Return {
        match env.get(id) {
            Some(type_) => type_.clone(),
            None => panic!("Missing id: {:?}", id),
        }
    }

    fn visit_if_expression(&mut self, env: Rc<Environment>, if_expression: &IfExpression) -> Type {
        let condition_type = if_expression.condition.accept(Rc::clone(&env), self);

        if condition_type != Type::Boolean {
            panic!("if condiditon must be of bool type");
        }

        let (true_path, false_path) = (
            if_expression.true_path.accept(Rc::clone(&env), self),
            if_expression.false_path.accept(env, self),
        );
        if true_path != false_path {
            panic!(
                "if branches differs in type, true leads to {:?} while false leads to {:?}",
                true_path, false_path
            );
        }

        true_path
    }
}

#[cfg(test)]
mod test {
    use super::AstVisitor;
    use super::Environment;
    use super::EnvironmentBuilder;
    use super::Rc;
    use super::TypeChecker;
    use crate::parser::ModuleParser;

    #[test]
    fn typecheck() {
        let code = "
        fn bool_fn(): bool => {
            let is_valid: bool = true;

            if not is_valid {
                true
            } else {
                bool_false()
            }
        };
        fn bool_false(): bool => false;
        fn noop_recursive() => noop_recursive();
        fn noop_cross_first() => noop_cross_seccond();
        fn noop_cross_seccond() => noop_cross_first();
        fn pass_by(something: i32): i32 => something;
        fn sum(lhs: i32, rhs: i32): i32 => lhs + pass_by(rhs);
        fn sum_with_body(lhs: i32, rhs: i32): i32 => {
            let x :i32 = lhs;
            let y :i32 = rhs + lhs - lhs *rhs;
            sum(x, y - y + 5_000_i32)
        };

        fn _and_(lhs: bool, rhs: bool): bool => {
            lhs && rhs || rhs and lhs or not (rhs or lhs) or (!rhs && not lhs)
        };
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = EnvironmentBuilder {};
        let mut type_checker = TypeChecker {};

        let env = Environment::new();
        let env = env_builder.visit_module_statement(env, &module);
        type_checker.visit_module_statement(Rc::new(env), &module);
    }

    #[test]
    #[should_panic(expected = "return statement is not inside a function")]
    fn return_as_module_statement() {
        use crate::parser::ModuleParser;
        let code = "
        let x: i32 = 5_i32;
        return x;
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = super::EnvironmentBuilder {};
        let mut type_checker = super::TypeChecker {};

        let env = Environment::new();
        let env = env_builder.visit_module_statement(env, &module);
        type_checker.visit_module_statement(Rc::new(env), &module);
    }

    #[test]
    #[should_panic(expected = "return statement is not inside a function")]
    fn return_as_module_statement_with_function() {
        use crate::parser::ModuleParser;
        let code = "
        fn foo(): i32 => 5_i32;
        return foo() + foo2();
        fn foo2(): i32 => 5_i32;
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = super::EnvironmentBuilder {};
        let mut type_checker = super::TypeChecker {};

        let env = Environment::new();
        let env = env_builder.visit_module_statement(env, &module);
        type_checker.visit_module_statement(Rc::new(env), &module);
    }

    #[test]
    #[should_panic(expected = "return statement differs in type, expected: I32, found: Boolean.")]
    fn return_inside_function_differs_in_type() {
        use crate::parser::ModuleParser;
        let code = "
        fn foo(): i32 => {
            if true {
                return 5_i32;
            } else {
                return true;
            }
        };
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = super::EnvironmentBuilder {};
        let mut type_checker = super::TypeChecker {};

        let env = Environment::new();
        let env = env_builder.visit_module_statement(env, &module);
        type_checker.visit_module_statement(Rc::new(env), &module);
    }

    #[test]
    #[should_panic(
        expected = "Function definition inside block is not allowed"
    )]
    fn function_def_inside_block() {
        use crate::parser::ModuleParser;
        let code = "
        fn foo(): i32 => {
            fn goo(): i64 => 5_i64;
            let x: i32 = 5_i32;
            if true {
                return 5_i32;
            } else {
                return 5_i32;
            }
        };
        ";

        let module = ModuleParser::new().parse(code).unwrap();

        let mut env_builder = super::EnvironmentBuilder {};
        let mut type_checker = super::TypeChecker {};

        let env = Environment::new();
        let env = env_builder.visit_module_statement(env, &module);
        type_checker.visit_module_statement(Rc::new(env), &module);
    }
}
