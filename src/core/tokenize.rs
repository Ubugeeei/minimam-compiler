#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: String,
}

#[derive(Debug)]
pub enum TokenKind {
    IntLiteral,
    Punct,
}

pub fn tokenize(exp_input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut source_index = 0;

    loop {
        let char = get_char(exp_input, &mut source_index);

        match char {
            Some(c) => match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let int_literal_string = read_int_literal(exp_input, &mut source_index);
                    tokens.push(Token {
                        kind: TokenKind::IntLiteral,
                        val: c.to_string() + &int_literal_string,
                    });
                }
                '+' | '-' | '*' | '/' | ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Punct,
                        val: c.to_string(),
                    });
                }
                ' ' | '\n' => {
                    continue;
                }
                _ => {
                    panic!("Invalid chars")
                }
            },
            None => {
                break;
            }
        }
    }

    return tokens;
}

pub fn get_token<'a>(tokens: &'a [Token], token_index: &'a mut usize) -> Option<&'a Token> {
    if *token_index == tokens.len() {
        return None;
    }
    let token = tokens.iter().nth(*token_index).unwrap();
    *token_index += 1;
    Some(token)
}

fn get_char(exp_input: &str, source_index: &mut usize) -> Option<char> {
    if *source_index >= exp_input.len() {
        return None;
    }
    let char = exp_input.chars().nth(*source_index).unwrap();
    *source_index += 1;
    Some(char)
}

fn unget_char(source_index: &mut usize) {
    *source_index -= 1;
}

fn read_int_literal(exp_input: &str, source_index: &mut usize) -> String {
    let mut result = String::new();

    let mut char = get_char(exp_input, source_index);
    loop {
        match char {
            Some(c) => {
                if c.is_digit(10) {
                    result.push(c);
                    char = get_char(exp_input, source_index);
                } else {
                    unget_char(source_index);
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    result
}
