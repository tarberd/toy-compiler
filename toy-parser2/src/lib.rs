pub mod ast;
mod grammar;

pub fn print_unexpected_token(input: &str, l: usize) {
    let mut current_line = 1;
    let mut current_collumn = 1;
    let mut found_line = 1;
    let mut found_collumn = 1;
    let mut current_line_start_offset = 0;
    let current_line_end_offset;
    let mut chars = input.char_indices();
    let mut found = false;
    let mut new_line = false;
    loop {
        match chars.next() {
            Some((index, c)) => {
                if index == l {
                    found_collumn = current_collumn;
                    found_line = current_line;
                    found = true;
                }

                match c {
                    '\n' => {
                        new_line = true;
                        if found {
                            current_line_end_offset = index;
                            break;
                        }
                    }
                    _ => {
                        if new_line {
                            current_line += 1;
                            current_collumn = 1;
                            current_line_start_offset = index;
                            new_line = false;
                        }
                        current_collumn += 1;
                    }
                };
            }
            None => {
                current_line_end_offset = current_line_start_offset + current_collumn;
                break;
            }
        }
    }
    println!("On line {} collumn {}", found_line, found_collumn);
    println!(
        "{}",
        &input[current_line_start_offset..current_line_end_offset]
    );
    println!("{:>width$}", "^", width = found_collumn);
    println!("Unexpected token");
}

pub fn parse_root_module(input: &str) -> Option<ast::RootModule> {
    let lexer = grammar::Lexer::new(input);
    match grammar::larlpop_grammar::RootModuleParser::new().parse(lexer) {
        Ok(module) => Some(module),
        Err(err) => {
            use lalrpop_util::ParseError;
            match err {
                ParseError::InvalidToken { location: _ } => {}
                ParseError::UnrecognizedEOF {
                    location: _,
                    expected: _,
                } => {}
                ParseError::UnrecognizedToken { token, expected: _ } => {
                    let (l, _token, _r) = token;
                    print_unexpected_token(input, l);
                }
                ParseError::ExtraToken { token } => {
                    let (l, _token, _r) = token;
                    print_unexpected_token(input, l);
                }
                ParseError::User { error: _ } => {}
            };
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
