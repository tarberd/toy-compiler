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
pub mod table;
