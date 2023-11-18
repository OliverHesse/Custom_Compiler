use std::collections::HashMap;
#[derive(Debug, Clone)]
//this should be a very simple lexer that just returns a value of 2
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
fn main() {
    let input:String = String::from("int main(){return 2;}");
    let var_result = lex(&input);
    match var_result {
        Ok(r)=>{
            println!("{:?}",r);
        },
        Err(_) => println!("An error occurd"),
    }
    
}
