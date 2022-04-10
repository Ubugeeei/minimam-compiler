use crate::core::{
    parse::{generate_expr, parse},
    tokenize::tokenize,
};

mod core;

fn main() {
    let tokens = tokenize(" 12 / 3 ; ");
    let expr = parse(&tokens, 0);
    generate_expr(&expr);
}