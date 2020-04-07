use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

pub mod ast;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        assert!(parser::ModuleParser::new().parse("fn main() => 42").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => -42").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => 42 + 10").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => -42 + 10").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => 42 + -10").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => -42 + -10").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => 42 * -10").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => 42 * -10 + 4 -3 +25").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main(x) => 2").is_ok());
    }
}
