use self::LiteralKind::*;
use self::TokenKind::*;
use std::str::Chars;

fn subslice_offset(outer: &str, inner: &str) -> usize {
    let self_beg = outer.as_ptr() as usize;
    let inner = inner.as_ptr() as usize;
    inner.wrapping_sub(self_beg)
}

#[derive(Debug, PartialEq)]
pub struct Token<'input> {
    pub kind: TokenKind<'input>,
    pub lexeme: &'input str,
}

impl<'input> Token<'input> {
    fn new(kind: TokenKind<'input>, lexeme: &'input str) -> Token<'input> {
        Token { kind, lexeme }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind<'input> {
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
    Literal(LiteralKind<'input>),

    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind<'input> {
    Integer {
        number: &'input str,
        suffix: Option<&'input str>,
    },
}

#[derive(Debug)]
pub struct Tokens<'input> {
    remainder: &'input str,
}

impl Tokens<'_> {
    pub fn new(input: &str) -> Tokens {
        Tokens { remainder: input }
    }
}

fn is_id_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || '_' == c
}

fn is_id_continue(c: char) -> bool {
    is_id_start(c) || ('0'..='9').contains(&c)
}

fn eat_identifier(chars: &mut Chars) {
    let mut chars_id = chars.clone();
    while let Some(c) = chars_id.next() {
        if is_id_continue(c) {
            chars.next();
        } else {
            break;
        }
    }
}

fn is_decimal_digit(c: char) -> bool {
    ('0'..='9').contains(&c) || '_' == c
}

fn eat_decimal_digits(chars: &mut Chars) {
    let mut chars_num = chars.clone();
    while let Some(c) = chars_num.next() {
        if is_decimal_digit(c) {
            chars.next();
        } else {
            break;
        }
    }
}

pub fn parse_token(input: &str) -> Option<(Token, &str)> {
    let mut chars = input.chars();
    let token_kind = match chars.next()? {
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
            let number_offset = subslice_offset(input, chars_num.as_str());
            let number = &input[0..number_offset];
            let suffix_offset = subslice_offset(input, chars_id.as_str());
            if number_offset < suffix_offset {
                Literal(Integer {
                    number,
                    suffix: Some(&input[number_offset..suffix_offset]),
                })
            } else {
                Literal(Integer {
                    number,
                    suffix: None,
                })
            }
        }
        _ => Unknown,
    };

    let offset = subslice_offset(input, chars.as_str());
    let token = Token::new(token_kind, &input[0..offset]);
    Some((token, &input[offset..]))
}

impl<'input> Iterator for Tokens<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, remainder) = parse_token(self.remainder)?;
        self.remainder = remainder;
        Some(token)
    }
}

pub fn tokens(input: &str) -> impl Iterator<Item = Token> {
    Tokens::new(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input = "";

        let mut tokens = Tokens::new(input);

        assert_eq!(tokens.next(), None);
    }

    fn test_single_token(input: &str, kind: TokenKind) {
        let expected_result = Token::new(kind, input);

        let mut tokens = Tokens::new(input);

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
                    number: &input,
                    suffix: None,
                }),
            );
        }
        let input = "0123456789";
        test_single_token(
            input,
            Literal(Integer {
                number: input,
                suffix: None,
            }),
        );
        let input = "9876543210123456789";
        test_single_token(
            input,
            Literal(Integer {
                number: input,
                suffix: None,
            }),
        );
        let input = "0______1_2_3_4_5_6_7_8_9_____";
        test_single_token(
            input,
            Literal(Integer {
                number: input,
                suffix: None,
            }),
        );
        let input = "0_AnI_d42_f";
        test_single_token(
            input,
            Literal(Integer {
                number: &input[0..2],
                suffix: Some(&input[2..]),
            }),
        );
        let input = "0_for";
        test_single_token(
            input,
            Literal(Integer {
                number: &input[0..2],
                suffix: Some(&input[2..]),
            }),
        );
    }
}
