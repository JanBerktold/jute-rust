use crate::Tokenizer;

#[derive(PartialEq, Debug)]
pub enum LexedToken<'a> {
    Identifier(&'a str),
    ModuleKeyword,
    ClassKeyword,
    LeftBracket,
    RightBracket,
    Semicolon,
}

pub struct Lexer<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Lexer<'a> {
        Lexer { tokenizer }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexedToken<'a>;

    fn next(&mut self) -> Option<LexedToken<'a>> {
        if let Some(token) = self.tokenizer.next() {
            let token = match token.text {
                "module" => LexedToken::ModuleKeyword,
                "class" => LexedToken::ClassKeyword,
                "{" => LexedToken::LeftBracket,
                "}" => LexedToken::RightBracket,
                ";" => LexedToken::Semicolon,
                _ => LexedToken::Identifier(token.text),
            };
            Some(token)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(text: &str, output: Vec<LexedToken>) {
        let mut tokenizer = Tokenizer::new(text);

        let mut lexer = Lexer {
            tokenizer: tokenizer,
        };

        for expected in output {
            if let Some(token) = lexer.next() {
                assert_eq!(token, expected);
            } else {
                assert!(false)
            }
        }

        if let Some(token) = lexer.next() {
            assert!(false)
        }
    }

    #[test]
    fn test_empty_string() {
        test_parse(&"", vec![])
    }

    #[test]
    fn test_single_field() {
        test_parse(
            &"long cxid;",
            vec![
                LexedToken::Identifier(&"long"),
                LexedToken::Identifier(&"cxid"),
                LexedToken::Semicolon,
            ],
        )
    }

    #[test]
    fn test_class() {
        let text = "
        class Id {
            ustring scheme;
            ustring id;
        }";
        test_parse(&text, vec![
            LexedToken::ClassKeyword,
            LexedToken::Identifier(&"Id"),
            LexedToken::LeftBracket,
            LexedToken::Identifier(&"ustring"),
            LexedToken::Identifier(&"scheme"),
            LexedToken::Semicolon,
            LexedToken::Identifier(&"ustring"),
            LexedToken::Identifier(&"id"),
            LexedToken::Semicolon,
            LexedToken::RightBracket,
        ])
    }
}
