mod lexical; // Importa o módulo lexer


use lexical::lexer::{Token,Lexer}; // Importa o Lexer e TokenType do módulo lexical/lexer.rs
fn main() {
    let input = r#"fn main() {
        let x = 42;
        if x > 0 {
            
        } else {
            
        }
        for i in 1..=5 {
            println!("{}", i);
        }
        return 0;
    }"#.to_string();

    let mut lexer = Lexer::new(input);

    loop {
        match lexer.next_token() {
            Some(token) => println!("{:?}", token),
            None => {
                println!("End of file");
                break;
            }
        }
    }
}