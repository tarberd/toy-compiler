mod grammar;
mod ast;

pub fn parse_root_module(input: &str) -> Option<ast::RootModule> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
