mod lexer;
mod token;
mod utils;

use lexer::Lexer;
use token::*;

fn main() {
    loop {
        let expr = utils::read_line("> ");
        let expr = expr.trim();

        println!("You entered: {}", expr);

        let mut lexer = Lexer::new(String::from(expr));
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);

            if let SyntaxTokenKind::EndOfFileToken = token.kind {
                break;
            }
        }
    }
}
