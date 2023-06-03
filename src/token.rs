
pub enum Token {
    Ident(String),
    String(String),
    Int(usize),
    Semicolon,
    OpenPar,
    ClosePar,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracker,
    Point,
    Assign,
    EqualTo,
    GreaterThan,
    LessThan,
    Add,
    Subtract,
    Mutliply,
    Divide
}
