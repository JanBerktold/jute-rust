pub enum LexedToken {
    Identifier(String),
    ModuleKeyword,
    ClassKeyword,
    LeftBracket,
    RightBracket,
    Semicolon,
}
