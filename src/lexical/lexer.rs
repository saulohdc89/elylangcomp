

use regex::Regex;


// Enumeração para os tipos de token
#[derive(Debug,PartialEq)]
pub enum Token {
    Int(String),
    Float(String),
    Char(String),
    String(String),
    Bool(String),
    Operator(String),
    OPTernary(String),
    Comparison(String),
    Assignment(String),
    IncrementDecrement(String),
    LogicalOperator(String),
    Identifier(String),
    NewLine(String),
    Function(String),
    Return,
    WhiteSpace(String),
    Keyword(String),
    EOF,
}



// analisador léxico
pub struct Lexer {
    input: String,
    current_position: usize,
}

impl Lexer {
     pub fn new(input: String) -> Self {
        Lexer {
            
            input,
           current_position: 0,
        }
    }
    
  
    

        pub fn next_token(&mut self) -> Option<Token> {
            if self.current_position >= self.input.len() {
                 return  Some(Token::EOF);
            }
            //Regex de  cada token
            let keyword_re = Regex::new(r"^(if|else|while|for|bool|int|string|char)\b").unwrap();
            let int_re = Regex::new(r"^\d+\b").unwrap();
            let float_re = Regex::new(r#"\d+\.\d+"#).unwrap();
            let string_literal_re = Regex::new(r#"^"(?:[^\\"]|\\t|\\n|\\")*""#).unwrap(); 
            let char_literal_re = Regex::new(r#"^'[^\\']'"#).unwrap();
            let bool_re = Regex::new(r"^(true|false)\b").unwrap();
            let comparison_operator_re = Regex::new(r"^(==|!=|<|>|<=|>=)").unwrap();
            let operator_re = Regex::new(r"^[-+*/%=]").unwrap(); //foi
            let incrementdecrement_re = Regex::new(r#"\+\+|\-\-"#).unwrap();
            let return_re = Regex::new(r"^(return)\b").unwrap();
            let function_re = Regex::new(r"^(fn)\b").unwrap();
            let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
            let logicalop_re: Regex = Regex::new(r"^(&&|\|\|)").unwrap();
            let whitespace_re: Regex = Regex::new(r"^\s+").unwrap(); // 
            let newline_re: Regex = Regex::new(r"^\n+").unwrap(); // 
            let input = &self.input[self.current_position..]; //foi
            let ternary_operator_re = Regex::new(   r"^\?").unwrap();
            let assignment_re = Regex::new(r"^=").unwrap();



            if let Some(caps) = keyword_re.captures(input){
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Keyword(token))
            }   else if let Some(caps) = operator_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Operator(token))
            }else if let Some(caps) = ternary_operator_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += 1;
                Some(Token::OPTernary(token))
            }
            else if let Some(caps) = bool_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Bool(token))
                
            }else if let Some(caps) = int_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Int(token))
            }else if let Some(caps) = float_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Float(token))
            }
            else if let Some(caps) = string_literal_re.captures(input) {
                let token = caps.get(1).unwrap().as_str().to_string();
                self.current_position += token.len() + 2; 
                Some(Token::String(token))
            }
            else if let Some(caps) = char_literal_re.captures(input) {
                let token = caps.get(1).unwrap().as_str().to_string();
                self.current_position += token.len() + 2;
                Some(Token::Char(token))
            }else if let Some(caps) = comparison_operator_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Comparison(token))
            }else if let Some(caps) = incrementdecrement_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::IncrementDecrement(token))
            }
            else if let Some(caps) = identifier_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Identifier(token))
            }
            else if let Some(caps) = logicalop_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::LogicalOperator(token))
            }
            else if let Some(caps) = whitespace_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::WhiteSpace(token))
            }
            else if let Some(caps) = assignment_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Assignment(token))
            }
            else if let Some(caps) = newline_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::NewLine(token))
            }

            else if let Some(_caps) = return_re.captures(input) {
                let token = "return".to_string();
                self.current_position += token.len();
                Some(Token::Return)
                }
            else if let Some(caps) = function_re.captures(input) {
                let token = caps.get(0).unwrap().as_str().to_string();
                self.current_position += token.len();
                Some(Token::Function(token))
            }
            else{
                None
            }
        }
    }









