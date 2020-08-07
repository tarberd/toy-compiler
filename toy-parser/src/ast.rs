#[derive(Debug)]
pub enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    ISize,
    USize,
    IntegerLiteral,

    Void,
    Pointer { type_id: Box<Type> },
    Array { type_id: Box<Type>, size: Expression },

    None,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;
        match (self, other) {
            (I8, I8) => true,
            (U8, U8) => true,
            (I16, I16) => true,
            (U16, U16) => true,
            (I32, I32) => true,
            (U32, U32) => true,
            (I64, I64) => true,
            (U64, U64) => true,
            (ISize, ISize) => true,
            (USize, USize) => true,
            (IntegerLiteral, IntegerLiteral) => true,
            (Void, Void) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ModuleStatement {
    pub id: Identifier,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct ExternFunctionDeclarationStatement {
    pub id: Identifier,
    pub parameters: Vec<(Identifier, Type)>,
    pub return_type: Type,
}

#[derive(Debug)]
pub struct FunctionDefinitionStatement {
    pub id: Identifier,
    pub parameters: Vec<(Identifier, Type)>,
    pub return_type: Type,
    pub body: Expression,
}

#[derive(Debug)]
pub struct VariableDefinitionStatement {
    pub id: Identifier,
    pub type_: Type,
    pub initialize_expression: Expression,
}

#[derive(Debug)]
pub enum Statement {
    Module(Box<ModuleStatement>),
    ExternFunctionDeclaration(Box<ExternFunctionDeclarationStatement>),
    FunctionDefinition(Box<FunctionDefinitionStatement>),
    VariableDefinition(Box<VariableDefinitionStatement>),
}

#[derive(Debug)]
pub struct BlockExpression {
    pub statements: Vec<Statement>,
    pub return_expression: Option<Expression>,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct AccessExpression {
    pub base: Expression,
    pub offset: Expression,
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub initialize_expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub value: isize,
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub enum Expression {
    Block(Box<BlockExpression>),
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Call(Box<CallExpression>),
    Access(Box<AccessExpression>),
    Array(ArrayLiteral),
    Integer(IntegerLiteral),
    Identifier(Identifier),
}

#[derive(Debug)]
pub enum Ast {
    Module {
        contents: Vec<Ast>,
    },
    FunctionDeclaration {
        id: Identifier,
        parameters: Vec<(Identifier, Type)>,
    },
    FunctionDefinition {
        id: String,
        parameters: Vec<(Identifier, Type)>,
        body: Box<Ast>,
    },
    VariableDefinition {
        id: Identifier,
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
        id: Identifier,
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
        id: Identifier,
    },
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Minus,
    Deref,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiplication,
    Division,
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Neg,
    Deref,
 }
