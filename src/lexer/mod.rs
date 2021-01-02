mod token;
pub use token::{SyntaxToken, SyntaxTokenKind};

pub struct Lexer {
    text: String,
    position: usize,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer { text, position: 0 }
    }

    pub fn get_current_char(&self) -> char {
        match self.text.chars().nth(self.position) {
            None => '\0',
            Some(character) => character,
        }
    }

    pub fn get_current_position_and_increment_position(&mut self) -> usize {
        let old = self.position;
        self.increment_position();
        old
    }

    pub fn increment_position(&mut self) {
        self.position += 1;
    }

    pub fn substr(&self, from: usize, to: usize) -> String {
        self.text.chars().skip(from).take(to).collect()
    }

    pub fn next_token(&mut self) -> SyntaxToken {
        if self.get_current_char().is_digit(10) {
            let start = self.position;
            while self.get_current_char().is_digit(10) {
                self.increment_position();
            }

            let raw_text = self.substr(start, self.position - start);
            let number: isize = raw_text.parse().expect("Error while parsing number token");

            return SyntaxToken {
                kind: SyntaxTokenKind::NumberToken(number),
                position: start,
                raw_text,
            };
        }

        if self.get_current_char().is_whitespace() {
            let start = self.position;
            while self.get_current_char().is_whitespace() {
                self.increment_position();
            }

            let raw_text = self.substr(start, self.position - start);

            return SyntaxToken {
                kind: SyntaxTokenKind::WhiteSpaceToken,
                position: start,
                raw_text,
            };
        }

        let curr = self.get_current_char();

        if curr == '\0' {
            return SyntaxToken {
                kind: SyntaxTokenKind::EndOfFileToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from('\0'),
            };
        }

        if curr == '+' {
            return SyntaxToken {
                kind: SyntaxTokenKind::PlusToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from("+"),
            };
        }

        if curr == '-' {
            return SyntaxToken {
                kind: SyntaxTokenKind::MinusToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from("-"),
            };
        }

        if curr == '*' {
            return SyntaxToken {
                kind: SyntaxTokenKind::StarToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from("*"),
            };
        }

        if curr == '/' {
            return SyntaxToken {
                kind: SyntaxTokenKind::SlashToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from("/"),
            };
        }

        if curr == '(' {
            return SyntaxToken {
                kind: SyntaxTokenKind::OpenParenthesisToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from("("),
            };
        }

        if curr == ')' {
            return SyntaxToken {
                kind: SyntaxTokenKind::CloseParenthesisToken,
                position: self.get_current_position_and_increment_position(),
                raw_text: String::from(")"),
            };
        }

        return SyntaxToken {
            kind: SyntaxTokenKind::BadToken,
            position: self.get_current_position_and_increment_position(),
            raw_text: String::from(curr),
        };
    }
}
