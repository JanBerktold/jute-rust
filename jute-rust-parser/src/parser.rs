use crate::Tokenizer;
use crate::{LexedToken, Lexer};

use std::result::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum PrimitiveFieldType {
    Boolean,
    Buffer,
    Byte,
    Double,
    Float,
    Int,
    Long,
    String,
    Custom(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum FieldType {
    Primitive(PrimitiveFieldType),
    Map(PrimitiveFieldType, PrimitiveFieldType),
    Vector(PrimitiveFieldType),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
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
        let tokenizer = Tokenizer::new(text);
        let lexer = Lexer::new(tokenizer);
        Parser { lexer }
    }
}

impl<'a> Parser<'a> {
    pub fn next(&mut self) -> Result<Module, String> {
        let mut module = Module {
            name: "".to_string(),
            classes: vec![],
        };

        if let Err(e) = self.expect_lexed_token(LexedToken::ModuleKeyword) {
            return Err(e.to_string());
        }

        if let Some(token) = self.lexer.next() {
            if let LexedToken::Identifier(name) = token {
                module.name = name.to_string();
            } else {
                return Err("something else".to_string());
            }
        } else {
            return Err("eof".to_string());
        }

        if let Err(e) = self.expect_lexed_token(LexedToken::LeftBracket) {
            return Err(e.to_string());
        }

        loop {
            if let Some(token) = self.lexer.next() {
                if let LexedToken::ClassKeyword = token {
                    let parsed_class = self.parse_class();
                    if let Ok(class) = parsed_class {
                        module.classes.push(class);
                    } else {
                        return Err(parsed_class.err().expect("logic error"));
                    }
                } else if let LexedToken::RightBracket = token {
                    return Ok(module);
                } else {
                    return Err("super_weird".to_string());
                }
            } else {
                return Err("ended too quickly".to_string());
            }
        }
    }

    fn parse_class(&mut self) -> Result<Class, String> {
        let mut class = Class {
            name: String::new(),
            fields: vec![],
        };

        if let Some(name_token) = self.lexer.next() {
            if let LexedToken::Identifier(name) = name_token {
                class.name = name.to_string();
            } else {
                return Err("bla".to_string());
            }
        } else {
            return Err("bla".to_string());
        }

        if let Err(e) = self.expect_lexed_token(LexedToken::LeftBracket) {
            return Err(e.to_string());
        }

        loop {
            if let Some(token) = self.lexer.next() {
                if let LexedToken::Identifier(property_type) = token {
                    if let Ok(field) = self.parse_field(property_type) {
                        class.fields.push(field);
                    } else {
                        return Err("bla".to_string());
                    }
                } else if let LexedToken::RightBracket = token {
                    return Ok(class);
                } else {
                    return Err("super weird stuff".to_string());
                }
            } else {
                return Err("bla".to_string());
            }
        }
    }

    fn parse_field(&mut self, field_type: &str) -> Result<Field, String> {
        let mut field = Field {
            field_type: self.parse_field_type(field_type),
            name: String::new(),
        };

        if let Some(token) = self.lexer.next() {
            if let LexedToken::Identifier(name) = token {
                field.name = name.to_string();
            } else {
                return Err("bla".to_string());
            }
        } else {
            return Err("bla".to_string());
        }

        if let Err(e) = self.expect_lexed_token(LexedToken::Semicolon) {
            return Err(e.to_string());
        }

        Ok(field)
    }

    fn parse_field_type(&self, field_type: &str) -> FieldType {
        match field_type {
            "boolean" => FieldType::Primitive(PrimitiveFieldType::Boolean),
            "buffer" => FieldType::Primitive(PrimitiveFieldType::Buffer),
            "byte" => FieldType::Primitive(PrimitiveFieldType::Byte),
            "double" => FieldType::Primitive(PrimitiveFieldType::Double),
            "float" => FieldType::Primitive(PrimitiveFieldType::Float),
            "int" => FieldType::Primitive(PrimitiveFieldType::Int),
            "long" => FieldType::Primitive(PrimitiveFieldType::Long),
            "ustring" => FieldType::Primitive(PrimitiveFieldType::String),
            other => {
                if other.starts_with("vector") {
                    let inner_type = &other[7..(other.len() - 1)];
                    if let FieldType::Primitive(primitive_type) = self.parse_field_type(inner_type) {
                        return FieldType::Vector(primitive_type);
                    }
                }
                FieldType::Primitive(PrimitiveFieldType::Custom(field_type.to_string()))
            }
        }
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
            match parser.next() {
                Ok(module) => {
                    assert_eq!(module, expected);
                }
                Err(e) => {
                    println!("{}", e);
                    assert!(false);
                }
            };
        }

        if let Ok(_token) = parser.next() {
            assert!(false)
        }
    }

    #[test]
    fn test_empty_string() {
        test_parse(&"", vec![])
    }

    #[test]
    fn test_complex_classes() {
        test_parse(
            "
module org.apache.zookeeper.txn {
    class TxnHeader {
        long clientId;
        int cxid;
        long zxid;
        long time;
        int type;
    }
    class CreateTxn {
        ustring path;
        buffer data;
        vector<org.apache.zookeeper.data.ACL> acl;
        boolean ephemeral;
        int parentCVersion;
    }
}

module org.apache.zookeeper.proto {
    class ConnectRequest {
        int protocolVersion;
        long lastZxidSeen;
        int timeOut;
        long sessionId;
        buffer passwd;
    }
    class SetWatches {
        long relativeZxid;
        vector<ustring>dataWatches;
        vector<ustring>existWatches;
        vector<ustring>childWatches;
    }        
    class SetDataResponse {
        org.apache.zookeeper.data.Stat stat;
    }
}
            ",
            vec![
                Module {
                    name: "org.apache.zookeeper.txn".to_string(),
                    classes: vec![
                        Class {
                            name: "TxnHeader".to_string(),
                            fields: vec![
                                Field {
                                    name: "clientId".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                                },
                                Field {
                                    name: "cxid".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                                },
                                Field {
                                    name: "zxid".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                                },
                                Field {
                                    name: "time".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                                },
                                Field {
                                    name: "type".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                                },
                            ],
                        },
                        Class {
                            name: "CreateTxn".to_string(),
                            fields: vec![
                                Field {
                                    name: "path".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::String),
                                },
                                Field {
                                    name: "data".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Buffer),
                                },
                                Field {
                                    name: "acl".to_string(),
                                    field_type: FieldType::Vector(PrimitiveFieldType::Custom(
                                        "org.apache.zookeeper.data.ACL".to_string(),
                                    )),
                                },
                                Field {
                                    name: "ephemeral".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Boolean),
                                },
                                Field {
                                    name: "parentCVersion".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                                },
                            ],
                        },
                    ],
                },
                Module {
                    name: "org.apache.zookeeper.proto".to_string(),
                    classes: vec![
                        Class {
                            name: "ConnectRequest".to_string(),
                            fields: vec![
                                Field {
                                    name: "protocolVersion".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                                },
                                Field {
                                    name: "lastZxidSeen".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                                },
                                Field {
                                    name: "timeOut".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                                },
                                Field {
                                    name: "sessionId".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                                },
                                Field {
                                    name: "passwd".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Buffer),
                                },
                            ],
                        },
                        Class {
                            name: "SetWatches".to_string(),
                            fields: vec![
                                Field {
                                    name: "relativeZxid".to_string(),
                                    field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                                },
                                Field {
                                    name: "dataWatches".to_string(),
                                    field_type: FieldType::Vector(PrimitiveFieldType::String),
                                },
                                Field {
                                    name: "existWatches".to_string(),
                                    field_type: FieldType::Vector(PrimitiveFieldType::String),
                                },
                                Field {
                                    name: "childWatches".to_string(),
                                    field_type: FieldType::Vector(PrimitiveFieldType::String),
                                },
                            ],
                        },
                        Class {
                            name: "SetDataResponse".to_string(),
                            fields: vec![Field {
                                name: "stat".to_string(),
                                field_type: FieldType::Primitive(PrimitiveFieldType::Custom("org.apache.zookeeper.data.Stat".to_string())),
                            }],
                        },
                    ],
                },
            ],
        )
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
            vec![Module {
                name: "org.apache.zookeeper.server.persistence".to_string(),
                classes: vec![Class {
                    name: "FileHeader".to_string(),
                    fields: vec![
                        Field {
                            name: "magic".to_string(),
                            field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                        },
                        Field {
                            name: "version".to_string(),
                            field_type: FieldType::Primitive(PrimitiveFieldType::Int),
                        },
                        Field {
                            name: "dbid".to_string(),
                            field_type: FieldType::Primitive(PrimitiveFieldType::Long),
                        },
                    ],
                }],
            }],
        )
    }
}
