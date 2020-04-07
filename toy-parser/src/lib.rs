use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(
        clippy::needless_lifetimes,
        clippy::clone_on_copy,
        clippy::needless_lifetimes,
        clippy::trivially_copy_pass_by_ref,
        clippy::too_many_arguments,
        clippy::redundant_static_lifetimes,
        clippy::new_without_default,
        clippy::let_and_return,
        clippy::inefficient_to_string,
    )]
    pub parser
);

pub mod ast;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        assert!(parser::ModuleParser::new().parse("fn main() => 42").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => -42").is_ok());
        assert!(parser::ModuleParser::new().parse("fn main() => 42 + 10") .is_ok());
        assert!(parser::ModuleParser::new() .parse("fn main() => -42 + 10") .is_ok());
        assert!(parser::ModuleParser::new() .parse("fn main() => 42 + -10") .is_ok());
        assert!(parser::ModuleParser::new() .parse("fn main() => -42 + -10") .is_ok());
        assert!(parser::ModuleParser::new() .parse("fn main() => 42 * -10") .is_ok());
        assert!(parser::ModuleParser::new() .parse("fn main() => 42 * -10 + 4 -3 +25") .is_ok());
        assert!(parser::ModuleParser::new().parse("fn main(x) => 2").is_ok());
    }
}
