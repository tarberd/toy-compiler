use lalrpop_util::lalrpop_mod;
use toy_lexer::{SpannedToken, SpannedTokens, Token};

lalrpop_mod!(
    #[allow(
        dead_code,
        clippy::eq_op,
        clippy::identity_op,
        clippy::just_underscores_and_digits,
        clippy::wrong_self_convention,
        clippy::unused_unit,
        clippy::unit_arg,
        clippy::needless_lifetimes,
        clippy::clone_on_copy,
        clippy::needless_lifetimes,
        clippy::trivially_copy_pass_by_ref,
        clippy::too_many_arguments,
        clippy::redundant_static_lifetimes,
        clippy::new_without_default,
        clippy::let_and_return,
        clippy::inefficient_to_string
    )]
    grammar,
    "/grammar/grammar.rs"
);

pub type Spanned<Token, Location, Error> = Result<(Location, Token, Location), Error>;

pub struct Lexer<'input> {
    tokens: SpannedTokens<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            tokens: SpannedTokens::new(input),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Spanned<Token, usize, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let SpannedToken { token, span } = self.tokens.next()?;
            if Token::WhiteSpace != token {
                return Some(Ok((span.offset, token, span.offset + span.len)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let src = "fn blerg() -> i32 { 5 }";

        let result = super::grammar::RootModuleParser::new()
            .parse(Lexer::new(src))
            .unwrap();
    }
}
