use toy_lexer::Span;

pub struct RootModule {
    pub functions: Vec<Function>,
}

impl RootModule {
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }
}

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

pub struct Parameter {
    pub id: Id,
    pub type_id: Id,
}

impl Parameter {
    pub fn new(id: Id, type_id: Id) -> Self {
        Self { id, type_id }
    }
}

pub struct Block {}

impl Block {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Id {
    pub id: Span,
}

impl Id {
    pub fn new(id: Span) -> Self {
        Self { id }
    }
}

pub struct Expression {
    pub kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self { kind }
    }
}

pub enum ExpressionKind {
    Block(Block),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Literal(Literal),
}

pub enum BinaryOperator {
    Sum,
}

pub enum Literal {
    Int(usize, IntKind),
}

pub enum IntKind {
    I32,
}
