use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

pub mod ast;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        assert!(parser::ModuleParser::new().parse("{}").is_ok());
        assert!(parser::ModuleParser::new().parse("{}").is_ok());
    }
}
