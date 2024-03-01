use std::iter::Peekable;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    PLUS(),
    MINUS(),
    DIV(),
    MULT(),
    MOD(),
    POW(),
    NUM(f32),
    OpenBracket(),
    CloseBracket(),
}

pub trait LexerMethods {
    fn tokenize(&self) -> Vec<Token>;
}
struct Lexer {
    input_string: String,
}
impl LexerMethods for Lexer {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        //let ch = self.input_string.chars();
        let mut chars = self.input_string.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    let mut current_num = String::from(c.clone());
                    if chars.peek().is_some() {
                        match chars.peek().unwrap() {
                            '0'..='9' => {
                                while let Some(digit) = chars.next() {
                                    current_num.push(digit.clone());
                                    if chars.peek().is_some() {
                                        match chars.peek().unwrap() {
                                            '0'..='9' => {}
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    //parse result
                    match current_num.parse::<f32>() {
                        Ok(n) => tokens.push(Token::NUM(n)),
                        Err(e) => panic!("something whent wrong with numbers"),
                    }
                }
                '+' => {
                    tokens.push(Token::PLUS());
                }
                '-' => {
                    tokens.push(Token::MINUS());
                }
                '*' => {
                    tokens.push(Token::MULT());
                }
                '/' => {
                    tokens.push(Token::DIV());
                }
                '^' => {
                    tokens.push(Token::POW());
                }
                '%' => {
                    tokens.push(Token::MOD());
                }
                '(' => {
                    tokens.push(Token::OpenBracket());
                }
                ')' => {
                    tokens.push(Token::CloseBracket());
                }
                _ => {
                    chars.next();
                }
            }
        }

        return tokens;
    }
}
#[derive(Debug, Clone)]
struct ASTNode {
    value: Token,
    left: Box<Option<ASTNode>>,
    right: Box<Option<ASTNode>>,
}
fn construct_tree(tokens: Vec<Token>) -> Option<ASTNode> {
    let mut current_token = tokens.iter().peekable();
    let root_node = third_order_op(&mut current_token);

    return root_node;
}
#[derive(Debug, Clone)]
enum factorEnum {
    num(f32),
    node(ASTNode),
}

fn first_order_op(current_token: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<ASTNode> {
    let factor_v = factor(current_token);

    if factor_v.is_none() {
        return None;
    }
    let factor_v = factor_v.unwrap();
    let mut node: Option<ASTNode> = Some(factor_v);
    let mut token: Option<Token> = None;

    while let Some(&c_token) = current_token.peek() {
        match c_token {
            Token::POW() => {
                token = Some((*c_token).clone());
                current_token.next();
                let right_factor = factor(current_token);

                if right_factor.is_none() {
                    return None;
                }

                node = Some(ASTNode {
                    left: Box::new(node),
                    value: token.unwrap(),
                    right: Box::new(Some(right_factor.unwrap())),
                });
            }
            _ => {
                break;
            }
        }
    }

    return node;
}
fn third_order_op(current_token: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<ASTNode> {
    let left_node = second_order_op(current_token);

    if left_node.is_none() {
        return None;
    }
    let left_node = left_node.unwrap();

    //same with multi and div but with plus and minus
    //in future change naming of functions to order of operations
    //like first order second order and third
    let mut token: Option<Token> = None;
    let mut root_node: Option<ASTNode> = Some(left_node.clone());

    while let Some(&c_token) = current_token.peek() {
        match c_token {
            Token::PLUS() | Token::MINUS() => {
                token = Some((*c_token).clone());
                current_token.next();

                let right_term = second_order_op(current_token);

                if right_term.is_none() {
                    return None;
                }
                let right_term = right_term.unwrap();

                root_node = Some(ASTNode {
                    left: Box::new(root_node),
                    value: token.unwrap(),
                    right: Box::new(Some(right_term)),
                });
            }
            _ => {
                break;
            }
        }
    }

    return root_node;
}

fn second_order_op(current_token: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<ASTNode> {
    let left_node = first_order_op(current_token);

    if left_node.is_none() {
        return None;
    }
    let left_node = left_node.unwrap();

    //same with multi and div but with plus and minus
    //in future change naming of functions to order of operations
    //like first order second order and third
    let mut token: Option<Token> = None;
    let mut root_node: Option<ASTNode> = Some(left_node.clone());

    while let Some(&c_token) = current_token.peek() {
        match c_token {
            Token::MULT() | Token::DIV() | Token::MOD() => {
                token = Some((*c_token).clone());
                current_token.next();

                let right_term = first_order_op(current_token);

                if right_term.is_none() {
                    return None;
                }
                let right_term = right_term.unwrap();

                root_node = Some(ASTNode {
                    left: Box::new(root_node),
                    value: token.unwrap(),
                    right: Box::new(Some(right_term)),
                });
            }
            _ => {
                break;
            }
        }
    }

    return root_node;
}

fn factor(current_token: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<ASTNode> {
    let token = Some(*current_token.peek().unwrap());

    match token.unwrap() {
        Token::NUM(v) => {
            current_token.next();
            let node = ASTNode {
                value: Token::NUM(v.clone()),
                left: Box::new(None),
                right: Box::new(None),
            };
            return Some(node);
        }
        Token::OpenBracket() => {
            current_token.next();
            let node = third_order_op(current_token);

            current_token.next();

            if node.is_none() {
                return None;
            }

            return node;
        }
        _ => {}
    }

    return Some(ASTNode {
        value: Token::NUM(-1.0),
        left: Box::new(None),
        right: Box::new(None),
    });
}

fn interpret(root: Option<ASTNode>) -> f32 {
    return visit(root);
}
fn visit(node: Option<ASTNode>) -> f32 {
    if node.is_some() {
        let node = node.unwrap();
        match node.value {
            Token::DIV() => {
                return visit(*node.left) / visit(*node.right);
            }
            Token::MULT() => return visit(*node.left) * visit(*node.right),
            Token::POW() => return visit(*node.left).powf(visit(*node.right)),
            Token::MOD() => return visit(*node.left) % visit(*node.right),
            Token::MINUS() => return visit(*node.left) - visit(*node.right),
            Token::PLUS() => return visit(*node.left) + visit(*node.right),
            Token::NUM(n) => {
                return n;
            }
            _ => {}
        }
    }
    return 2.0;
}

fn main() {
    //(33*3-30*2+2324)/2
    let mut temp_input = String::from("(33*3-30*2+2324)/2");
    let mut lexer = Lexer {
        input_string: temp_input,
    };
    let tokens = lexer.tokenize();
    let root = construct_tree(tokens);

    println!("i got an answer of:");
    println!("{}", interpret(root));
}
