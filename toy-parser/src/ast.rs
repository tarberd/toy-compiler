pub enum Ast {
    Module{
        contents: Box<Ast>,
    },
    Function{
        id: String,
        body: Box<Ast>,
    },
    None,
}
