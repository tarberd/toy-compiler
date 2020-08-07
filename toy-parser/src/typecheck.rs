use crate::ast::*;
use crate::visitor::{AstVisitor, Visitable};

type TypeCheckerResult = Result<Type, String>;

struct TypeChecker {}

impl TypeChecker {
    fn type_of(&self, id: &Identifier) -> Type {
        Type::I32
    }
}

impl AstVisitor<Type> for TypeChecker {
    type Return = Type;

    fn visit_module_statement(&mut self, m: &ModuleStatement) -> Self::Return {
        m.accept(self)
    }

    fn visit_statement(&mut self, s: &Statement) -> Self::Return {
        s.accept(self)
    }

    fn visit_extern_function_declaration_statement(
        &mut self,
        f: &ExternFunctionDeclarationStatement,
    ) -> Self::Return {
        f.accept(self)
    }

    fn visit_function_definition_statement(
        &mut self,
        function: &FunctionDefinitionStatement,
    ) -> Self::Return {
        function.accept(self)
    }

    fn visit_variable_definition_statement(
        &mut self,
        variable: &VariableDefinitionStatement,
    ) -> Self::Return {
        variable.accept(self)
    }

    fn visit_expression(&mut self, expression: &Expression) -> Self::Return {
        expression.accept(self)
    }

    fn visit_block_expression(&mut self, block: &BlockExpression) -> Self::Return {
        Type::I32
    }

    fn visit_unary_expression(&mut self, unary: &UnaryExpression) -> Self::Return {
        Type::I32
    }

    fn visit_binary_expression(&mut self, binary: &BinaryExpression) -> Self::Return {
        Type::I32
    }

    fn visit_call_expression(&mut self, call: &CallExpression) -> Self::Return {
        Type::I32
    }

    fn visit_access_expression(&mut self, access: &AccessExpression) -> Self::Return {
        let base_type = access.base.accept(self);

        match base_type {
            Type::Array { type_id, size: _ } => *type_id,
            _ => panic!("access type error"),
        }
    }

    fn visit_array_literal(&mut self, array: &ArrayLiteral) -> Self::Return {
        let expr_types: Vec<Type> = array
            .initialize_expressions
            .iter()
            .map(|expression| expression.accept(self))
            .collect();

        match expr_types.first() {
            Some(nested_type) => {
                for expr_type in expr_types.iter().next() {
                    if nested_type != expr_type {
                        panic!("expression type difer in array init expression");
                    }
                }

                Type::Array {
                    type_id: Box::new(*nested_type),
                    size: Expression::Integer(IntegerLiteral { value: 0 }),
                }
            }
            None => Type::Array {
                type_id: Box::new(Type::None),
                size: Expression::Integer(IntegerLiteral { value: 0 }),
            },
        }
    }

    fn visit_integer_literal(&mut self, _: &IntegerLiteral) -> Self::Return {
        Type::IntegerLiteral
    }

    fn visit_identifier(&mut self, id: &Identifier) -> Self::Return {
        self.type_of(id)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn read_example_file_and_typecheck() {}
}
