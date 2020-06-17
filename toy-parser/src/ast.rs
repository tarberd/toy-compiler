#[derive(Debug)]
pub enum Ast {
    Module {
        contents: Vec<Ast>,
    },
    FunctionDeclaration {
        id: String,
        parameters: Vec<String>,
    },
    FunctionDefinition {
        id: String,
        parameters: Vec<String>,
        body: Box<Ast>,
    },
    VariableDefinition {
        id: String,
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
    IntegerLiteral {
        value: i32,
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
