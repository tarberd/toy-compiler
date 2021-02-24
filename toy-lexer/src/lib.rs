use self::LiteralKind::*;
use self::Token::*;
use std::str::{CharIndices, Chars};

fn subslice_offset(outer: &str, inner: &str) -> usize {
    let self_beg = outer.as_ptr() as usize;
    let inner = inner.as_ptr() as usize;
    inner.wrapping_sub(self_beg)
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct Span {
    pub offset: usize,
    pub len: usize,
}

impl Span {
    pub fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }
}

#[derive(Debug, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    /// ";"
    Semicolon,
    /// ":"
    Colon,
    /// ","
    Comma,
    /// "("
    OpenParentheses,
    /// ")"
    CloseParentheses,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "="
    Equals,
    /// "=="
    DoubleEquals,
    /// "=>"
    FatArrow,
    /// "->"
    Arrow,
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Asterisk,
    /// "/"
    Slash,
    /// "%"
    Percent,
    /// "fn"
    Function,
    /// "let"
    Let,
    /// "an idendifier"
    Identifier,
    Literal(LiteralKind),

    WhiteSpace,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Integer { suffix_offset: Option<usize> },
}

#[derive(Debug)]
pub struct SpannedTokens<'input> {
    remainder: CharIndices<'input>,
}

impl<'input> SpannedTokens<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            remainder: input.char_indices(),
        }
    }
}

impl<'input> Iterator for SpannedTokens<'input> {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<Self::Item> {
        let (initial, _) = self.remainder.clone().nth(0)?;
        let (token, lexeme, _) = parse_token(self.remainder.as_str())?;
        self.remainder.nth(lexeme.len() - 1);
        Some(SpannedToken::new(token, Span::new(initial, lexeme.len())))
    }
}

fn is_id_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || '_' == c
}

fn is_id_continue(c: char) -> bool {
    is_id_start(c) || ('0'..='9').contains(&c)
}

fn is_decimal_digit(c: char) -> bool {
    ('0'..='9').contains(&c) || '_' == c
}

fn is_whitespace(c: char) -> bool {
    ' ' == c || '\t' == c || '\n' == c
}

fn eat_while(chars: &mut Chars, predicate: fn(char) -> bool) {
    let mut chars_id = chars.clone();
    while let Some(c) = chars_id.next() {
        if predicate(c) {
            chars.next();
        } else {
            break;
        }
    }
}

fn eat_identifier(chars: &mut Chars) {
    eat_while(chars, is_id_continue);
}

fn eat_decimal_digits(chars: &mut Chars) {
    eat_while(chars, is_decimal_digit);
}

fn eat_whitespace(chars: &mut Chars) {
    eat_while(chars, is_whitespace);
}

/// returns (token parsed, lexeme of the token parsed, remainder)
pub fn parse_token(input: &str) -> Option<(Token, &str, &str)> {
    let mut chars = input.chars();
    let token = match chars.next()? {
        c if is_whitespace(c) => {
            eat_whitespace(&mut chars);
            WhiteSpace
        }
        ';' => Semicolon,
        ',' => Comma,
        ':' => Colon,
        '(' => OpenParentheses,
        ')' => CloseParentheses,
        '{' => OpenBrace,
        '}' => CloseBrace,
        '+' => Plus,
        '-' => match chars.clone().next() {
            Some('>') => {
                chars.next();
                Arrow
            }
            _ => Minus,
        },
        '*' => Asterisk,
        '/' => Slash,
        '%' => Percent,
        '=' => match chars.clone().next() {
            Some('=') => {
                chars.next();
                DoubleEquals
            }
            Some('>') => {
                chars.next();
                FatArrow
            }
            _ => Equals,
        },
        c if is_id_start(c) => {
            eat_identifier(&mut chars);
            let offset = subslice_offset(input, chars.as_str());
            match &input[0..offset] {
                "fn" => Function,
                "let" => Let,
                _ => Identifier,
            }
        }
        _c @ '0'..='9' => {
            let mut chars_num = chars.clone();
            eat_decimal_digits(&mut chars_num);
            chars = chars_num.clone();
            let mut chars_id = chars_num.clone();
            if let Some(c) = chars_id.next() {
                if is_id_start(c) {
                    eat_identifier(&mut chars_id);
                    chars = chars_id.clone();
                }
            }
            let suffix_offset = subslice_offset(input, chars_num.as_str());
            let token_offset = subslice_offset(input, chars.as_str());
            if suffix_offset < token_offset {
                Literal(Integer {
                    suffix_offset: Some(suffix_offset),
                })
            } else {
                Literal(Integer {
                    suffix_offset: None,
                })
            }
        }
        _ => Unknown,
    };

    let offset = subslice_offset(input, chars.as_str());
    Some((token, &input[0..offset], &input[offset..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input = "";

        let mut tokens = SpannedTokens::new(input);

        assert_eq!(tokens.next(), None);
    }

    fn test_single_token(input: &str, token: Token) {
        let expected_result = SpannedToken::new(token, Span::new(0, input.len()));

        let mut tokens = SpannedTokens::new(input);

        assert_eq!(tokens.next(), Some(expected_result));
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn single_key_tokens() {
        test_single_token(";", Semicolon);
        test_single_token(",", Comma);
        test_single_token(":", Colon);
        test_single_token("(", OpenParentheses);
        test_single_token(")", CloseParentheses);
        test_single_token("{", OpenBrace);
        test_single_token("}", CloseBrace);
        test_single_token("=", Equals);
        test_single_token("==", DoubleEquals);
        test_single_token("=>", FatArrow);
        test_single_token("->", Arrow);
        test_single_token("fn", Function);
        test_single_token("let", Let);
    }

    #[test]
    fn parse_single_identifiers() {
        for c in 'a'..='z' {
            test_single_token(&c.to_string(), Identifier);
        }
        for c in 'A'..='Z' {
            test_single_token(&c.to_string(), Identifier);
        }
        test_single_token("_", Identifier);
        test_single_token("_a_aZz1234LASdj_kf___", Identifier);
        test_single_token("_0", Identifier);
    }

    #[test]
    fn parse_single_numbers() {
        for c in '0'..='9' {
            let input = c.to_string();
            test_single_token(
                &input,
                Literal(Integer {
                    suffix_offset: None,
                }),
            );
        }
        let input = "0123456789";
        test_single_token(
            input,
            Literal(Integer {
                suffix_offset: None,
            }),
        );
        let input = "9876543210123456789";
        test_single_token(
            input,
            Literal(Integer {
                suffix_offset: None,
            }),
        );
        let input = "0______1_2_3_4_5_6_7_8_9_____";
        test_single_token(
            input,
            Literal(Integer {
                suffix_offset: None,
            }),
        );
        let input = "0_AnI_d42_f";
        test_single_token(
            input,
            Literal(Integer {
                suffix_offset: Some(2),
            }),
        );
        let input = "0_for";
        test_single_token(
            input,
            Literal(Integer {
                suffix_offset: Some(2),
            }),
        );
        let input = "01234_for";
        test_single_token(
            input,
            Literal(Integer {
                suffix_offset: Some(6),
            }),
        );
    }

    #[test]
    fn test_function() {
        let src = "fn blerg() -> i32 { 5 }";

        let expected = vec![
            SpannedToken::new(Function, Span::new(0, 2)),
            SpannedToken::new(WhiteSpace, Span::new(2, 1)),
            SpannedToken::new(Identifier, Span::new(3, 5)),
            SpannedToken::new(OpenParentheses, Span::new(8, 1)),
            SpannedToken::new(CloseParentheses, Span::new(9, 1)),
            SpannedToken::new(WhiteSpace, Span::new(10, 1)),
            SpannedToken::new(Arrow, Span::new(11, 2)),
            SpannedToken::new(WhiteSpace, Span::new(13, 1)),
            SpannedToken::new(Identifier, Span::new(14, 3)),
            SpannedToken::new(WhiteSpace, Span::new(17, 1)),
            SpannedToken::new(OpenBrace, Span::new(18, 1)),
            SpannedToken::new(WhiteSpace, Span::new(19, 1)),
            SpannedToken::new(
                Literal(LiteralKind::Integer {
                    suffix_offset: None,
                }),
                Span::new(20, 1),
            ),
            SpannedToken::new(WhiteSpace, Span::new(21, 1)),
            SpannedToken::new(CloseBrace, Span::new(22, 1)),
        ];

        let result: Vec<SpannedToken> = SpannedTokens::new(src).collect();

        for (result, expected) in result.iter().zip(expected) {
            assert_eq!(*result, expected);
        }
    }
}
