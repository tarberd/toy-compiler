use toy_lexer::Span;

#[derive(Debug)]
pub struct RootModule {
    pub functions: Vec<Function>,
}

impl RootModule {
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }
}

#[derive(Debug)]
pub struct Function {
    pub id: Id,
    pub return_id: Id,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

impl Function {
    pub fn new(id: Id, parameters: Vec<Parameter>, return_id: Id, body: Block) -> Self {
        Self {
            id,
            parameters,
            return_id,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub id: Id,
    pub type_id: Id,
}

impl Parameter {
    pub fn new(id: Id, type_id: Id) -> Self {
        Self { id, type_id }
    }
}

#[derive(Debug)]
pub struct Block {
    pub expression: Expression,
}

impl Block {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct Id {
    pub span: Span,
}

impl Id {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Block(Box<Block>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Literal(Literal),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Sum,
}

#[derive(Debug)]
pub struct Literal {
    pub literal: Span,
    pub suffix: Option<Span>,
}

impl Literal {
    pub fn new(literal: Span, suffix: Option<Span>) -> Self {
        Self {literal, suffix}
    }
}
