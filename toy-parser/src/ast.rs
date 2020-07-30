#[derive(Debug)]
pub enum Type {
    I32,
    Pointer { type_id: Box<Type> },
    Array { type_id: Box<Type>, size: Box<Ast> },
}

#[derive(Debug)]
pub enum Ast {
    Module {
        contents: Vec<Ast>,
    },
    FunctionDeclaration {
        id: String,
        parameters: Vec<(String, Type)>,
    },
    FunctionDefinition {
        id: String,
        parameters: Vec<(String, Type)>,
        body: Box<Ast>,
    },
    VariableDefinition {
        id: String,
        type_id: Type,
        expression: Box<Ast>,
    },
    BlockExpression {
        statements: Vec<Ast>,
        return_expression: Box<Ast>,
    },
    UnaryExpression {
        operator: Operator,
        expression: Box<Ast>,
    },
    BinaryExpression {
        operator: Operator,
        left: Box<Ast>,
        right: Box<Ast>,
    },
    CallExpression {
        id: String,
        arguments: Vec<Ast>,
    },
    AccessExpression {
        base: Box<Ast>,
        offset: Box<Ast>,
    },
    IntegerLiteral {
        value: i32,
    },
    ArrayLiteral {
        values: Vec<Ast>,
    },
    Identifier {
        id: String,
    },
    None,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Neg,
    Deref,
}
