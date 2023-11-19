use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum Token{
    Num(u32),
    Id(String),
    Identifier(String),
    Keyword(String),
    OpenBrace(char),
    CloseBrace(char),
    OpenBracket(char),
    CloseBracket(char),
    SemiColon(char),
    Assign(char),
}
#[derive(Debug)]
pub struct  Program{
    funcs: Vec<Function>,
}
#[derive(Debug)]
pub struct Function{
    Type:String,
    Name:String,
    Statements: Vec<Statement>
}
#[derive(Debug)]
pub struct Statement{
    KeyWord : String,
    Expres: Expression,
}
#[derive(Debug)]
pub struct Expression{
    data: u32,
}
fn lex(input: &String) -> Result<Vec<Token>,String>{
    //turn input into a list of chars
    let mut result = Vec::new();

    let mut words = HashMap::new(); 
    let mut it = input.chars().peekable();
    let mut _lineno = 1;


    while let Some(&c) = it.peek(){
        match c {
            ' ' | '\t' =>{
                it.next();
            },
            '\n' =>{
                _lineno += 1;
                it.next();
            }
            'i' =>{
                it.next();
                let ch = it.peek();
                if let Some('n') = ch{
                    it.next();
                    let ch = it.peek();
                    if let Some('t') = ch{
                        result.push(Token::Keyword("int".to_string()));
                        it.next();
                    }
                }
            }
            '0'..='9' => {
                println!("started an int");
                let mut n = c.to_string().parse::<u32>().expect("Character not a digit");

                it.next();
                let mut digit_char = it.peek();

                while let Some(&i) = digit_char{
                    if i.is_digit(10){
                        let digit = i.to_string().parse::<u32>().expect("Character not a digit");
                        n = n*10 + digit;
                        it.next();
                        digit_char = it.peek();
                    } else  {
                        digit_char = None;}
                }
                println!("finished an int");
                result.push(Token::Num(n));
            }
            //used to target characters
            'A'..='Z' | 'a'..='z' =>{
                let mut s = String::new();
                s.push(c);
                it.next();
                let mut ch = it.peek();
                while let Some(&i) = ch{
                    if !i.is_digit(10) && !i.is_alphabetic(){
                        ch = None;
                    }else{
                        s.push(i);
                        it.next();
                        ch = it.peek();
                    }
                }
                println!("{}",s);
                result.push(Token::Id(s.clone()));
                words.insert(s.clone(),Token::Id(s.clone()));

            },
            '{' => {result.push(Token::OpenBrace(c)); it.next();},
            '}' => {result.push(Token::CloseBrace(c)); it.next();},
            '(' => {result.push(Token::OpenBracket(c)); it.next();},
            ')' => {result.push(Token::CloseBracket(c)); it.next();},
            ';' => {result.push(Token::SemiColon(c)); it.next();},
            _ => {
                result.push(Token::Id(c.to_string()));{
                    it.next();
                }
            }
        }
    }
    return Ok(result);
}

fn parse(lex : Vec<Token>) -> Result<Program,String>{
    let mut result = Program { funcs: Vec::new() };
    let mut token_iter = lex.iter().peekable();
    
    while let Some(&token) = token_iter.peek(){
        match token{
            Token::Keyword(_)=>{
                let Type = (*token).clone();

                token_iter.next();
                let token = *token_iter.peek().unwrap();
                match token {
                    Token::Id(_)=>{
                        //now i know that the next 3 tokens should be (){ so lets check quickly if that is true
                        let name = (*token).clone();
                        token_iter.next();
                        let token = *token_iter.peek().unwrap();
                        match token {
                            Token::OpenBracket('(')=>{
                                token_iter.next();
                                let token = *token_iter.peek().unwrap();
                                match token {
                                    Token::CloseBracket(')')=>{
                                        token_iter.next();
                                        let token = *token_iter.peek().unwrap();
                                        match token{
                                            Token::OpenBrace('{') =>{
                                                // a function has been declared here
                                                let mut New_Type:String = String::new();
                                                let mut new_name:String = String::new();
                                                match name {
                                                    Token::Id(t)=>{new_name = t},
                                                    _=>{}
                                                }
                                                match Type {
                                                    Token::Keyword(t)=>{New_Type = t},
                                                    _=>{}
                                                }
                                                
                                                let mut all_states = Vec::<Statement>::new();
                                                // could from here do a while until i detect a } to check for End of func
                                                while let Some(&token) = token_iter.peek(){
                                                    match token {
                                                        Token::Id(_) =>{
                                                            let id = (*token).clone();
                                                            token_iter.next();
                                                            let token = *token_iter.peek().unwrap();
                                                            match token{
                                                                Token::Num(_)=>{
                                                                    let num = (*token).clone();
                                                                    token_iter.next();
                                                                    let token = *token_iter.peek().unwrap();
                                                                    match token{
                                                                        Token::SemiColon(_)=>{
                                                                            //end of statement found
                                                                            if let Token::Num(new_num) = num{
                                                                                if let Token::Id(new_id) = id{
                                                                                    let new_statement = Statement{KeyWord:new_id,Expres:Expression { data: new_num }};
                                                                                    all_states.push(new_statement);
                                                                                }else{}
                                                                            }else{}
                                                                        }
                                                                        _=>{}
                                                                    }
                                                                },
                                                                _=>{}
                                                            }
                                                        },
                                                        Token::CloseBrace(_)=>{
                                                            break
                                                        }
                                                        _=>{token_iter.next();}
                                                    }
                                                }
                                                let new_func = Function{Type:New_Type,Name:new_name,Statements:all_states};
                                                result.funcs.push(new_func);
                                            },

                                            _=>{}
                                        }
                                    },
                                    _=>{}
                                }
                            },
                            _ =>{}
                        }
                    },
                    _=>{}
                }
            },

            _ =>{
                token_iter.next();
            }
        }
    }
    Ok(result)
}
fn main() {
    let input:String = String::from("int main(){return 2;}");
    let var_result = lex(&input);
    match var_result {
        Ok(r)=>{
            println!("{:?}",r);
            let parse_result = parse(r);
            match parse_result{
                Ok(v)=>{
                    println!("{:?}",v);
                },
                Err(_) => println!("ann error happend"),
            }
        },
        Err(_) => println!("An error occurd"),
    }
    
}
