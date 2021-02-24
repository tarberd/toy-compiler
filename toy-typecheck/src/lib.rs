use std::collections::BTreeMap;
use toy_lexer::Span;
use toy_parser2::ast::{
    BinaryOperator, Block, Expression, ExpressionKind, Function, Literal, RootModule,
};

pub fn typecheck_root_module(source: &str, module: &RootModule) {
    let env_builder = EnvironmentBuilder::new(source);
    env_builder.typecheck_root_module(module);
}

#[derive(Debug)]
pub enum Type {
    Fn(Vec<Type>, Box<Type>),
    I32,
}

#[derive(Debug)]
pub struct Environment {
    map: BTreeMap<Span, Type>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, span: Span, ty: Type) {
        self.map.insert(span, ty);
    }

    pub fn make_scope() -> Environment {
        Environment::new()
    }
}

pub struct EnvironmentBuilder<'source> {
    source: &'source str,
    type_builder: TypeBuilder<'source>,
}

impl<'source> EnvironmentBuilder<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            type_builder: TypeBuilder::new(source),
        }
    }

    pub fn typecheck_root_module(&self, module: &RootModule) {
        let env = self.build_environment(module);

        for function in &module.functions {
            self.typecheck_function(&env, function);
        }
    }

    pub fn typecheck_function(&self, env: &Environment, function: &Function) {
        let _ty = self.typecheck_block(env, &function.body);
    }

    pub fn typecheck_block(&self, env: &Environment, block: &Block) -> Type {
        self.typecheck_expression(env, &block.expression)
    }

    pub fn typecheck_expression(&self, env: &Environment, expr: &Expression) -> Type {
        match &expr.kind {
            ExpressionKind::Block(block) => self.typecheck_block(env, block),
            ExpressionKind::Binary(lhs, op, rhs) => {
                self.typecheck_binary_operation(env, lhs, op, rhs)
            }
            ExpressionKind::Literal(lit) => self.typecheck_literal(env, lit),
        }
    }

    pub fn typecheck_binary_operation(
        &self,
        env: &Environment,
        lhs: &Expression,
        _op: &BinaryOperator,
        rhs: &Expression,
    ) -> Type {
        let lhs_ty = self.typecheck_expression(env, lhs);
        let _rhs_ty = self.typecheck_expression(env, rhs);
        //check if operatin is a valid function on the environment
        lhs_ty
    }

    pub fn typecheck_literal(&self, _env: &Environment, literal: &Literal) -> Type {
        match literal.suffix {
            Some(suffix) => self.type_builder.build_type_from_span(suffix),
            None => Type::I32,
        }
    }

    pub fn build_environment(&self, module: &RootModule) -> Environment {
        let mut env = Environment::new();

        for function in &module.functions {
            let ty = self.type_builder.build_function_type(function);
            env.bind(function.id.span, ty);
        }

        Environment::new()
    }
}

struct TypeBuilder<'source> {
    source: &'source str,
}

impl<'source> TypeBuilder<'source> {
    pub fn new(source: &'source str) -> Self {
        Self { source }
    }

    fn span_to_str(&self, span: Span) -> &str {
        &self.source[span.offset..span.offset + span.len]
    }

    pub fn build_type_from_span(&self, span: Span) -> Type {
        match self.span_to_str(span) {
            "i32" => Type::I32,
            s => panic!("{}: not a valid type identifier", s),
        }
    }

    pub fn build_function_type(&self, function: &Function) -> Type {
        let mut parameters = Vec::with_capacity(function.parameters.len());
        for parameter in &function.parameters {
            parameters.push(self.build_type_from_span(parameter.type_id.span))
        }
        Type::Fn(
            parameters,
            Box::new(self.build_type_from_span(function.return_id.span)),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
