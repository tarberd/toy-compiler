use crate::ast::*;

pub trait AstVisitor<Return> {
    type Return;
    fn visit_module_statement(&mut self, m: &ModuleStatement) -> Return;
    fn visit_statement(&mut self, s: &Statement) -> Return;
    fn visit_extern_function_declaration_statement(
        &mut self,
        f: &ExternFunctionDeclarationStatement,
    ) -> Return;
    fn visit_function_definition_statement(
        &mut self,
        function: &FunctionDefinitionStatement,
    ) -> Return;
    fn visit_variable_definition_statement(
        &mut self,
        variable: &VariableDefinitionStatement,
    ) -> Return;

    fn visit_expression(&mut self, expression: &Expression) -> Return;
    fn visit_block_expression(&mut self, block: &BlockExpression) -> Return;
    fn visit_unary_expression(&mut self, unary: &UnaryExpression) -> Return;
    fn visit_binary_expression(&mut self, binary: &BinaryExpression) -> Return;
    fn visit_call_expression(&mut self, call: &CallExpression) -> Return;
    fn visit_access_expression(&mut self, access: &AccessExpression) -> Return;

    fn visit_array_literal(&mut self, array: &ArrayLiteral) -> Return;
    fn visit_integer_literal(&mut self, literal: &IntegerLiteral) -> Return;
    fn visit_identifier(&mut self, id: &Identifier) -> Return;
}

pub trait Visitable<V: AstVisitor<Return>, Return> {
    fn accept(&self, visitor: &mut V) -> Return;
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for Statement {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_statement(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for ModuleStatement {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_module_statement(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return>
    for ExternFunctionDeclarationStatement
{
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_extern_function_declaration_statement(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for FunctionDefinitionStatement {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_function_definition_statement(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for VariableDefinitionStatement {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_variable_definition_statement(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for Expression {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_expression(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for BlockExpression {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_block_expression(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for UnaryExpression {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_unary_expression(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for BinaryExpression {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_binary_expression(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for CallExpression {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_call_expression(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for AccessExpression {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_access_expression(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for ArrayLiteral {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_array_literal(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for IntegerLiteral {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_integer_literal(self)
    }
}

impl<V: AstVisitor<Return>, Return> Visitable<V, Return> for Identifier {
    fn accept(&self, visitor: &mut V) -> Return {
        visitor.visit_identifier(self)
    }
}

// pub fn walk_statement<V: Visitor>(visitor: &mut V, statement: &Statement) {}
//
// pub fn walk_module_statement<V: Visitor>(v: &mut V, m: &ModuleStatement) {
//     for statement in &m.statements {
//         v.visit_statement(statement);
//     }
// }
//
// pub fn walk_extern_function_declaration_statement<V: Visitor>(
//     _visitor: &mut V,
//     _extern_function: &ExternFunctionDeclarationStatement,
// ) {
// }
//
// pub fn walk_function_definition_statement<V: Visitor>(
//     visitor: &mut V,
//     function: &FunctionDefinitionStatement,
// ) {
//     visitor.visit_expression(&function.body);
// }
//
// pub fn walk_variable_definition_statement<V: Visitor>(
//     visitor: &mut V,
//     variable: &VariableDefinitionStatement,
// ) {
//     visitor.visit_expression(&variable.initialize_expression);
// }
//
// pub fn walk_expression<V: Visitor>(v: &mut V, expression: &Expression) {
//     use Expression::*;
//
//     match expression {
//         Block(block) => v.visit_block_expression(block),
//         Unary(unary) => v.visit_unary_expression(unary),
//         Binary(binary) => v.visit_binary_expression(binary),
//         Call(call) => v.visit_call_expression(call),
//         Access(access) => v.visit_access_expression(access),
//         Array(array) => v.visit_array_expression(array),
//         Integer(num) => v.visit_integer_expression(*num),
//         Identifier(id) => v.visit_identifier_expression(id),
//     }
// }
//
// pub fn walk_block_expresion<V: Visitor>(v: &mut V, block: &BlockExpression) {
//     for statement in &block.statements {
//         v.visit_statement(&statement);
//     }
//     if let Some(expr) = &block.return_expression {
//         v.visit_expression(expr);
//     }
// }
//
// pub fn walk_unary_expression<V: Visitor>(v: &mut V, unary: &UnaryExpression) {
//     v.visit_expression(&unary.expression);
// }
//
// pub fn walk_binary_expression<V: Visitor>(v: &mut V, binary: &BinaryExpression) {
//     v.visit_expression(&binary.left);
//     v.visit_expression(&binary.right);
// }
//
// pub fn walk_call_expression<V: Visitor>(v: &mut V, call: &CallExpression) {
//     v.visit_expression(&call.callee);
//     for argument in &call.arguments {
//         v.visit_expression(argument);
//     }
// }
//
// pub fn walk_access_expression<V: Visitor>(v: &mut V, access: &AccessExpression) {
//     v.visit_expression(&access.base);
//     v.visit_expression(&access.offset);
// }
//
// pub fn walk_array_expression<V: Visitor>(v: &mut V, array: &ArrayExpression) {
//     for expression in &array.initialize_expressions {
//         v.visit_expression(expression);
//     }
// }
