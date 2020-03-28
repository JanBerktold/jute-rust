use crate::{Lexer, LexedToken};
use crate::Tokenizer;

use std::result::Result;
use std::fs;

pub enum FieldType {
    Boolean,
    Buffer,
    Byte,
    Double,
    Float,
    Int,
    Long,
    Map(Box<FieldType>, Box<FieldType>),
    String,
    Vector(Box<FieldType>),
    Custom(String),
}

pub struct Field {
    pub name: String,
    pub field_type: FieldType,

}

#[derive(Debug, Eq, PartialEq)]
pub struct Class {
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Module {
    pub name: String,
    pub classes: Vec<Class>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn from_string(text: &'a str) -> Parser<'a> {
       let mut tokenizer = Tokenizer::new(text);
       let mut lexer = Lexer::new(tokenizer);
       Parser { lexer }
    }
}

impl<'a> Parser<'a> {
    fn next(&mut self) -> Result<Module, &str> {
        let mut module = Module{
            name: "".to_string(),
            classes: vec![],
        };

        if let Err(e) = self.expect_lexed_token(LexedToken::ModuleKeyword) {
            return Err(e);
        }

        if let Some(token) = self.lexer.next() {
            if let LexedToken::Identifier(name) = token{
                module.name = name.to_string();
            } else {
                return Err("something else");
            }
        } else {
            return Err("eof");
        }

        Ok(module)
    }

    fn expect_lexed_token(&mut self, expected_token: LexedToken) -> Result<LexedToken, &str> {
        if let Some(token) = self.lexer.next() {
            if token == expected_token {
                Ok(token)
            } else {
            Err("got wrong token") 

            }
        } else {
           Err("got wrong token") 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(text: &str, output: Vec<Module>) {
        let mut parser = Parser::from_string(text);

        for expected in output {
            if let Ok(module) = parser.next() {
                assert_eq!(module, expected);
            } else {
                assert!(false)
            }
        }

        if let Ok(token) = parser.next() {
            assert!(false)
        }
    }

    #[test]
    fn test_empty_string() {
        test_parse(&"", vec![])
    }

    #[test]
    fn test_single_class() {
        test_parse(
"
module org.apache.zookeeper.server.persistence {
    class FileHeader {
        int magic;
        int version;
        long dbid;
    }
}
",

            vec![
            ],
        )
    }
}
