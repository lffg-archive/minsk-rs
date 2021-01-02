mod lexer;
mod utils;

use lexer::{Lexer, SyntaxTokenKind};

fn main() {
    loop {
        let expr = utils::io::read_line("> ");
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
