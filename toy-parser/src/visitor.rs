use crate::ast::*;

pub trait AstVisitor<Environment, Return> {
    type Return;
    type Environment;

    fn visit_module_statement(&mut self, env: Environment, m: &ModuleStatement) -> Return;
    fn visit_statement(&mut self, env: Environment, s: &Statement) -> Return;
    fn visit_extern_function_declaration_statement(
        &mut self,
        env: Environment,
        f: &ExternFunctionDeclarationStatement,
    ) -> Return;
    fn visit_function_definition_statement(
        &mut self,
        env: Environment,
        function: &FunctionDefinitionStatement,
    ) -> Return;
    fn visit_variable_definition_statement(
        &mut self,
        env: Environment,
        variable: &VariableDefinitionStatement,
    ) -> Return;
    fn visit_return_statement(
        &mut self,
        env: Environment,
        return_statement: &ReturnStatement,
    ) -> Return;

    fn visit_expression(&mut self, env: Environment, expression: &Expression) -> Return;
    fn visit_block_expression(&mut self, env: Environment, block: &BlockExpression) -> Return;
    fn visit_if_expression(&mut self, env: Environment, if_expression: &IfExpression) -> Return;
    fn visit_unary_expression(&mut self, env: Environment, unary: &UnaryExpression) -> Return;
    fn visit_binary_expression(&mut self, env: Environment, binary: &BinaryExpression) -> Return;
    fn visit_call_expression(&mut self, env: Environment, call: &CallExpression) -> Return;
    fn visit_access_expression(&mut self, env: Environment, access: &AccessExpression) -> Return;

    fn visit_identifier(&mut self, env: Environment, id: &Identifier) -> Return;
}

pub trait Visitable<V: AstVisitor<Environment, Return>, Environment, Return> {
    fn accept(&self, env: Environment, visitor: &mut V) -> Return;
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for Statement
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_statement(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for ModuleStatement
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_module_statement(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for ExternFunctionDeclarationStatement
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_extern_function_declaration_statement(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for FunctionDefinitionStatement
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_function_definition_statement(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for VariableDefinitionStatement
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_variable_definition_statement(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for ReturnStatement
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_return_statement(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for Expression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for BlockExpression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_block_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for IfExpression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_if_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for UnaryExpression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_unary_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for BinaryExpression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_binary_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for CallExpression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_call_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for AccessExpression
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_access_expression(env, self)
    }
}

impl<V: AstVisitor<Environment, Return>, Environment, Return> Visitable<V, Environment, Return>
    for Identifier
{
    fn accept(&self, env: Environment, visitor: &mut V) -> Return {
        visitor.visit_identifier(env, self)
    }
}
