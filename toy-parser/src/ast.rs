#[derive(Clone, Debug)]
pub enum Type {
    Boolean,

    Int(IntType),
    UInt(UIntType),

    Void,
    Pointer {
        type_id: Box<Type>,
    },

    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },

    None,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(u128, LiteralIntType),
    Boolean(bool),
}

impl Literal {
    pub fn type_(&self) -> Type {
        match self {
            Literal::Int(_, t) => match t {
                LiteralIntType::Signed(t) => Type::Int(t.clone()),
                LiteralIntType::Unsigned(t) => Type::UInt(t.clone()),
                LiteralIntType::Unsufixed => Type::None,
            },
            Literal::Boolean(_) => Type::Boolean,
        }
    }
}

#[derive(Clone, Debug)]
pub enum LiteralIntType {
    Signed(IntType),
    Unsigned(UIntType),
    Unsufixed,
}

#[derive(Clone, Debug)]
pub enum IntType {
    ISize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Clone, Debug)]
pub enum UIntType {
    USize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;
        match (self, other) {
            (Boolean, Boolean) => true,
            (Int(_), Int(_)) => true,
            (UInt(_), UInt(_)) => true,
            (Void, Void) => true,
            (
                Pointer { type_id: type_self },
                Pointer {
                    type_id: type_other,
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
    Identifier(Identifier),
    Literal(Literal),
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
