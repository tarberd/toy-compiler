pub enum Type {
    Integer,
}

pub enum Ast {
    Module { contents: Box<Ast> },
    Function(FunctionDeclaration),
    ExpressionBlock(ExpressionBlock),
    None,
}

pub struct ExpressionBlock {
    pub return_expression: i32,
}

pub struct FunctionDeclaration {
    pub id: String,
    pub body: ExpressionBlock,
}
