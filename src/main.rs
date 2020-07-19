use std::env;

enum TokenKind {
    Num,
    Operator,
}

struct Token {
    kind: TokenKind,
    val: String,
}

fn lexing(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut begin_idx = 0;
    let mut is_in_the_middle_of_number = false;
    for (i, s) in input.chars().enumerate() {
        match s {
            ' ' => {
                if is_in_the_middle_of_number {
                    let new_token = Token {
                        kind: TokenKind::Num,
                        val: input[begin_idx..i].to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i + 1;
                    is_in_the_middle_of_number = false;
                } else {
                    begin_idx = i + 1;
                }
            }
            '+' => {
                if is_in_the_middle_of_number {
                    let new_token = Token {
                        kind: TokenKind::Num,
                        val: input[begin_idx..i].to_string(),
                    };
                    tokens.push(new_token);
                    let new_token = Token {
                        kind: TokenKind::Operator,
                        val: String::from("+"),
                    };
                    tokens.push(new_token);
                    begin_idx = i + 1;
                    is_in_the_middle_of_number = false;
                } else {
                    let new_token = Token {
                        kind: TokenKind::Operator,
                        val: String::from("+"),
                    };
                    tokens.push(new_token);
                    begin_idx = i + 1;
                }
            }
            '0'..='9' => {
                if is_in_the_middle_of_number {
                } else {
                    begin_idx = i;
                    is_in_the_middle_of_number = true;
                }
            }
            _ => {
                panic!("Cannot tokenize {}", s);
            }
        }
    }

    tokens
}

fn parsing(tokens: &Vec<Token>) -> Vec<String> {
    let mut commands = Vec::new();
    commands.push(String::from(".global main"));
    commands.push(String::from("main:"));

    let mut stack: Vec<&Token> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if i == 0 {
            match token.kind {
                TokenKind::Num => {
                    commands.push(format!("\tli a0, {}", token.val));
                }
                TokenKind::Operator => panic!("Expect a number in the head"),
            }
            continue;
        }

        match token.kind {
            TokenKind::Num => {
                let top = match stack.pop() {
                    Some(top) => top,
                    None => panic!("Expect an operator after a number"),
                };
                match top.kind {
                    TokenKind::Num => panic!("Expect an operator after a number"),
                    TokenKind::Operator => {
                        commands.push(format!("\tadd a0, a0, {}", token.val));
                    }
                }
            }
            TokenKind::Operator => {
                if stack.is_empty() {
                    stack.push(&token);
                } else {
                    panic!("Expect a number after an operator");
                }
                continue;
            }
        }
    }

    commands.push(format!("\tret"));
    commands
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = args[1..].join(" ");
    let input: String = [&input, " "].join("");

    let tokens = lexing(&input);
    let commands = parsing(&tokens);

    for command in commands.iter() {
        println!("{}", command);
    }
}
