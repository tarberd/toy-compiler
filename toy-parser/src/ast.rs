#[derive(Clone, Debug)]
pub enum Type {
    Boolean,

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
    Pointer {
        type_id: Box<Type>,
    },
    Array {
        type_id: Box<Type>,
        size: Expression,
    },

    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },

    None,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;
        match (self, other) {
            (Boolean, Boolean) => true,
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
            (
                Pointer { type_id: type_self },
                Pointer {
                    type_id: type_other,
                },
            ) => type_self.eq(type_other),
            (
                Array {
                    type_id: type_self,
                    size: _,
                },
                Array {
                    type_id: type_other,
                    size: _,
                },
            ) => type_self.eq(type_other),
            (
                Function {
                    parameters,
                    return_type,
                },
                Function {
                    parameters: parameter_rhs,
                    return_type: return_type_rhs,
                },
            ) => parameters == parameter_rhs && return_type.eq(return_type_rhs),
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleStatement {
    pub id: Identifier,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct ExternFunctionDeclarationStatement {
    pub id: Identifier,
    pub parameters: Vec<(Identifier, Type)>,
    pub return_type: Type,
}

#[derive(Clone, Debug)]
pub struct FunctionDefinitionStatement {
    pub id: Identifier,
    pub parameters: Vec<(Identifier, Type)>,
    pub return_type: Type,
    pub body: Expression,
}

#[derive(Clone, Debug)]
pub struct VariableDefinitionStatement {
    pub id: Identifier,
    pub type_: Type,
    pub initialize_expression: Expression,
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Module(Box<ModuleStatement>),
    ExternFunctionDeclaration(Box<ExternFunctionDeclarationStatement>),
    FunctionDefinition(Box<FunctionDefinitionStatement>),
    VariableDefinition(Box<VariableDefinitionStatement>),
    Return(Box<ReturnStatement>),
}

#[derive(Clone, Debug)]
pub struct BlockExpression {
    pub statements: Vec<Statement>,
    pub return_expression: Option<Expression>,
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub expression: Expression,
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone, Debug)]
pub struct IfExpression {
    pub condition: Expression,
    pub true_path: Expression,
    pub false_path: Expression,
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub struct AccessExpression {
    pub base: Expression,
    pub offset: Expression,
}

#[derive(Clone, Debug)]
pub struct ArrayLiteral {
    pub initialize_expressions: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub struct IntegerLiteral {
    pub value: isize,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Identifier {
    pub value: String,
}

impl From<String> for Identifier {
    fn from(string: String) -> Self {
        Identifier { value: string }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Block(Box<BlockExpression>),
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    If(Box<IfExpression>),
    Call(Box<CallExpression>),
    Access(Box<AccessExpression>),
    Array(ArrayLiteral),
    Integer(Box<IntegerLiteral>),
    Boolean(BooleanLiteral),
    Identifier(Identifier),
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Not,
    Minus,
    Deref,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    GreaterThan,
    GreaterEqualThan,
    Equal,
    LessEqualThan,
    LessThan,

    And,
    Or,
    Plus,
    Minus,
    Multiplication,
    Division,
}
