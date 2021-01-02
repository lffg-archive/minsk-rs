#[derive(Debug)]
pub enum SyntaxTokenKind {
    NumberToken(isize),
    WhiteSpaceToken,

    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    OpenParenthesisToken,
    CloseParenthesisToken,

    EndOfFileToken,
    BadToken,
}

#[derive(Debug)]
pub struct SyntaxToken {
    pub kind: SyntaxTokenKind,
    pub raw_text: String,
    pub position: usize,
}
