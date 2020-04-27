#[derive(Debug)]
pub enum Ast {
    Module {
        contents: Vec<Ast>,
    },
    FunctionDeclaration {
        id: String,
        body: Expression,
        parameters: Vec<String>,
    },
    Expression(Expression),
    None,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Neg,
}

#[derive(Debug)]
pub enum Expression {
    Block {
        return_expression: Box<Expression>,
    },
    Unary {
        operator: Operator,
        expression: Box<Expression>,
    },
    Binary {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call {
        id: String,
        arguments: Vec<Expression>,
    },
    IntegerLiteral {
        value: i32,
    },
    Identifier {
        id: String,
    },
}
