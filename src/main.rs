use std::iter::Peekable;

#[derive(Debug,Clone,Copy)]
pub enum Token {
    PLUS(),
    MINUS(),
    DIV(),
    MULT(),
    MOD(),
    POW(),
    NUM(i32),
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
                    match current_num.parse::<i32>() {
                        Ok(n) => tokens.push(Token::NUM(n)),
                        Err(e) => panic!("something whent wrong with numbers"),
                    }
                    println!("{} is a numnber", current_num);
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
                _ => {chars.next();}
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
fn construct_tree(tokens: Vec<Token>) ->Option<ASTNode> {
    let mut current_token = tokens.iter().peekable();
    let root_node = expr(&mut current_token);
    println!("{:?}",current_token.peek());
    return root_node 
}
#[derive(Debug,Clone)]
enum factorEnum {
    num(i32),
    node(ASTNode),
}
fn power() {}

fn expr(current_token: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<ASTNode> {
    println!("==============in expr==========");
    println!("{:?}",current_token.peek());
    let left_node = term(current_token);
    println!("==============in expr==========");
    if left_node.is_none(){
        return None;
    }
    let left_node = left_node.unwrap();
    
    //same with multi and div but with plus and minus
    //in future change naming of functions to order of operations
    //like first order second order and third
    let mut token:Option<Token> = None;
    let mut root_node:Option<ASTNode> = Some(left_node.clone());
    
    println!("yoyoyo");
    while let Some(&c_token) = current_token.peek() {
        println!("hi");
        println!("{:?}",c_token);
        match c_token {
            Token::PLUS() | Token::MINUS() =>{
                println!("test");
                token = Some((*c_token).clone());
                current_token.next();
                println!("{:?}",current_token.peek());
                let right_term = term(current_token);
                println!("==============in expr==========");
                if right_term.is_none(){
                   return None;
                }
                let right_term = right_term.unwrap();
               
                root_node = Some(ASTNode {
                    left: Box::new(root_node),
                    value: token.unwrap(),
                    right: Box::new(Some(right_term)),
                });  
                
                println!("CREATED NEW NODE {:?}",root_node);          
                
            }
            _=>{
                break;}
            
        }
        
    }
    
    return root_node;
}

fn term(current_token: &mut Peekable<std::slice::Iter<'_, Token>>)->Option<ASTNode> {
    println!("==============in term=============");
    println!("{:?}",current_token.peek());
    let factor_v = factor(current_token);
    println!("==============in term=============");
    println!("enterd from here");
    if factor_v.is_none(){
        return None;
    }
    let factor_v = factor_v.unwrap();
    let mut node:Option<ASTNode> = Some(factor_v);
    let mut token: Option<Token> = None;
   
    while let Some(&c_token) = current_token.peek() {
        println!("{:?}",current_token.peek());
        match c_token {
            Token::MULT() | Token::DIV() => {
                
                token = Some((*c_token).clone());
                current_token.next();
                let right_factor = factor(current_token);
                
                println!("==============in term=============");
                println!("{:?}",right_factor);
                if right_factor.is_none(){
                    return None
                }
                
                node = Some(ASTNode {
                    left: Box::new(node),
                    value: token.unwrap(),
                    right: Box::new(Some(right_factor.unwrap())),
                });
                println!("CREATED NEW NODE {:?}",node);
                
            },
            _ => {
                println!("i should not be here");
                println!("{:?}",current_token.peek());
                break
            }
        }

    }
    println!("{:?}",node);
    return node;
}

fn factor(current_token: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<ASTNode> {
    println!("=========in factor===========");
    println!("{:?}",current_token.peek());
    let token = Some(*current_token.peek().unwrap());
    
    
    match token.unwrap(){
        Token::NUM(v) => {
            println!("removing num");
            current_token.next();
            let node = ASTNode{
                value:Token::NUM(v.clone()),
                left:Box::new(None),
                right:Box::new(None),
            };
            return Some(node);
        },
        Token::OpenBracket() => {
            println!("=============STARTING BRACKETS============");
            current_token.next();
            let node = expr(current_token);
            println!("=========in factor===========");
            println!("{:?}",current_token.peek());
            current_token.next();
            
            println!("=============ENDING BRACKETS============");
            if node.is_none(){
                println!("Node here is causing issues");
                return None;
            }
            
            return node;
        }
        _ => {println!("this should not be called");}
    }

    return Some(ASTNode{
        value:Token::NUM(-1),
        left:Box::new(None),
        right:Box::new(None),
    });
}
fn main() {
    //(33*3-30*2+2324)/2
    let mut temp_input = String::from("3*2+2");
    let mut lexer = Lexer {
        input_string: temp_input,
    };
    let tokens = lexer.tokenize();
    let root = construct_tree(tokens);
    println!("test");
    println!("{:?}",root);
}
