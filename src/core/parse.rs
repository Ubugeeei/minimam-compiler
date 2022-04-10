use super::tokenize::{get_token, Token, TokenKind};

#[derive(Debug)]
pub struct Expr {
    kind: ExprKind,
    val: isize,
    operator: Operator,
    operand: Option<Box<Expr>>,
    left: Option<Box<Expr>>,
    right: Option<Box<Expr>>,
}

#[derive(Debug)]
pub enum ExprKind {
    IntLiteral,
    Unary,
    Binary,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    None,
}

pub fn generate_expr(expr: &Expr) {
    println!("global _main");
    println!("_main:");
    match expr.kind {
        ExprKind::IntLiteral => println!("    mov rcx, {}", expr.val),

        ExprKind::Unary => match expr.operator {
            Operator::Plus => println!("    mov rcx, {}", expr.operand.as_ref().unwrap().val),
            Operator::Minus => println!("    mov rcx, -{}", expr.operand.as_ref().unwrap().val),
            _ => panic!("unexpected operator"),
        },

        ExprKind::Binary => match expr.operator {
            Operator::Plus => {
                println!("    mov rax, {}", expr.left.as_ref().unwrap().val);
                println!("    mov rcx, {}", expr.right.as_ref().unwrap().val);
                println!("    add rax, rcx");
            }
            Operator::Minus => {
                println!("    mov rax, {}", expr.left.as_ref().unwrap().val);
                println!("    mov rcx, {}", expr.right.as_ref().unwrap().val);
                println!("    sub rax, rcx");
            }
            Operator::Multiply => {
                println!("    mov rax, {}", expr.left.as_ref().unwrap().val);
                println!("    mov rcx, {}", expr.right.as_ref().unwrap().val);
                println!("    mul rcx");
            }
            Operator::Divide => {
                println!("    mov ax, {}", expr.left.as_ref().unwrap().val);
                println!("    mov dx, {}", expr.right.as_ref().unwrap().val);
                println!("    div dl");
            }
            Operator::None => panic!("unexpected operator"),
        },
        _ => panic!("unexpected token"),
    }
    // println!("    add rax, rcx");
    println!("    ret");
}

pub fn parse(tokens: &[Token], token_index: usize) -> Expr {
    let mut _token_index = token_index;
    let expr = parse_unary_expression(tokens, &mut _token_index);
    let token = get_token(tokens, &mut _token_index);
    match token {
        Some(t) => {
            if t.val == String::from(";") {
                expr
            } else if t.val == String::from("+")
                || t.val == String::from("-")
                || t.val == String::from("*")
                || t.val == String::from("/")
            {
                Expr {
                    kind: ExprKind::Binary,
                    val: 0,
                    operator: match t.val.as_str() {
                        "+" => Operator::Plus,
                        "-" => Operator::Minus,
                        "*" => Operator::Multiply,
                        "/" => Operator::Divide,
                        _ => panic!("unexpected operator"),
                    },
                    operand: None,
                    left: Some(Box::new(expr)),
                    right: Some(Box::new(parse(tokens, _token_index))),
                }
            } else {
                panic!("unexpected token");
            }
        }
        None => expr,
    }
}

pub fn parse_unary_expression(tokens: &[Token], token_index: &mut usize) -> Expr {
    let token = get_token(tokens, token_index);
    match token {
        Some(token) => match token.kind {
            TokenKind::IntLiteral => Expr {
                kind: ExprKind::IntLiteral,
                val: token.val.parse().unwrap(),
                operator: Operator::None,
                operand: None,
                left: None,
                right: None,
            },

            TokenKind::Punct => {
                let operator = match token.val.as_str() {
                    "+" => Operator::Plus,
                    "-" => Operator::Minus,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    _ => panic!("Invalid operator"),
                };

                Expr {
                    kind: ExprKind::Unary,
                    val: 0,
                    operator,
                    operand: Some(Box::new(parse_unary_expression(tokens, token_index))),
                    left: None,
                    right: None,
                }
            }

            _ => panic!("unexpected token"),
        },

        None => panic!("unexpected token"),
    }
}
