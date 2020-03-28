use std::iter::Peekable;
use std::str::CharIndices;

pub struct Token<'a> {
    pub text: &'a str,
}

pub struct Tokenizer<'a> {
    text: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            text: text,
            iter: text.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let mut begin: usize = 0;
        let mut end: usize = 0;

        loop {
            let peek = self.iter.peek();

            if peek.is_none() {
                if begin < end {
                    return Some(Token {
                        text: &self.text[begin..end],
                    });
                } else {
                    return None;
                }
            }

            if let Some(y) = peek {
                let (i, c) = y;

                if c.is_whitespace() {
                    if begin < end {
                        return Some(Token {
                            text: &self.text[begin..end],
                        });
                    }
                    begin = *i + 1;
                    end = *i + 1;
                } else if c == &';' {
                    if begin < end {
                        return Some(Token {
                            text: &self.text[begin..end],
                        });
                    }

                    begin = *i;
                    end = *i + 1;
                } else {
                    end = *i + 1;
                }

                self.iter.next();

                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(text: &str, output: Vec<&str>) {
        let mut tokenizer = Tokenizer::new(text);

        for expected in output {
            if let Some(token) = tokenizer.next() {
                assert_eq!(token.text, expected);
            } else {
                assert!(false)
            }
        }

        if let Some(token) = tokenizer.next() {
            assert!(false)
        }
    }

    #[test]
    fn test_empty_string() {
        test_parse(&"", vec![])
    }

    #[test]
    fn test_single_field() {
        test_parse(&"long cxid;", vec![&"long", &"cxid", &";"])
    }

    #[test]
    fn test_class() {
        let text = "
        class Id {
            ustring scheme;
            ustring id;
        }";
        test_parse(
            &text,
            vec![
                &"class", &"Id", &"{", &"ustring", &"scheme", &";", &"ustring", &"id", &";", &"}",
            ],
        )
    }
}
