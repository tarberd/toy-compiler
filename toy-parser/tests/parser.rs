// integration tests should be in /tests folder

#[cfg(test)]
mod tests {
  use toy_parser::parser::ModuleParser;

    #[test]
    fn parses_trivial_integer_functions () {
        assert!(ModuleParser::new().parse("fn main() => 42").is_ok());
        assert!(ModuleParser::new().parse("fn main() => -42").is_ok());
    }

    #[test]
    fn parses_unary_and_binary_expressions () {
        assert!(ModuleParser::new().parse("fn main() => 42 + 10").is_ok());
        assert!(ModuleParser::new().parse("fn main() => -42 + 10").is_ok());
        assert!(ModuleParser::new().parse("fn main() => 42 + -10").is_ok());
        assert!(ModuleParser::new().parse("fn main() => -42 + -10").is_ok());
        assert!(ModuleParser::new().parse("fn main() => 42 * -10").is_ok());
        assert!(ModuleParser::new().parse("fn main() => 42 * -10 + 4 -3 +25").is_ok());
    }

    #[test]
    fn parses_argument_function () {
        assert!(ModuleParser::new().parse("fn main(x) => 2").is_ok());
        assert!(ModuleParser::new().parse("fn main(x) => x").is_ok());
        assert!(ModuleParser::new().parse("fn main(x) => x * 2").is_ok());
    }
}
