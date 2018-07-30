#[derive(Debug, PartialEq)]
pub enum Token {
    LPAREN(TokenInfo),
    RPAREN(TokenInfo),
    LBRACE(TokenInfo),
    RBRACE(TokenInfo),
    WORD(TokenInfo, String),
    NUM(TokenInfo, i32),
    STRING(TokenInfo, String),
}

pub struct Tokenizer<'a> {
    buf: Vec<char>,
    chars: &'a mut Iterator<Item = char>,
    col: usize,
    line: usize,
}

#[derive(Debug, PartialEq)]
pub struct TokenInfo {
    line: usize,
    column: usize,
}

impl<'a> Tokenizer<'a> {
    fn info(&self) -> TokenInfo {
        TokenInfo {
            line: self.line,
            column: self.col,
        }
    }

    fn word_or_token(&mut self, token: Token, c: char) -> Token {
        if self.buf.is_empty() {
            token
        } else {
            let result = self.word();
            self.buf.push(c);
            result
        }
    }

    fn word(&mut self) -> Token {
        let result: String = self.buf.drain(..).collect();
        if let Ok(num) = result.parse() {
            Token::NUM(self.info(), num)
        } else {
            Token::WORD(self.info(), result)
        }
    }

    fn quoted_string(&mut self) -> Token {
        let mut escape = false;
        while let Some(c) = self.chars.next() {
            self.col += 1;
            match c {
                '"' if !escape => break,
                '"' => {
                    escape = false;
                    self.buf.push(c)
                }
                '\\' if !escape => escape = true,
                '\\' => {
                    escape = false;
                    self.buf.push(c)
                }
                '\n' => {
                    self.nextline();
                    self.buf.push(c);
                }
                _ => self.buf.push(c),
            }
        }
        let result: String = self.buf.drain(..).collect();
        Token::STRING(self.info(), result)
    }

    fn nextline(&mut self) {
        self.line += 1;
        self.col = 0;
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            self.col += 1;

            match c {
                '(' => return { let info = self.info(); Some(self.word_or_token(Token::LPAREN(info), '('))},
                ')' => return  { let info = self.info(); Some(self.word_or_token(Token::LPAREN(info), ')'))},
                '{' => return  { let info = self.info(); Some(self.word_or_token(Token::LBRACE(info), '{'))},
                '}' => return  { let info = self.info(); Some(self.word_or_token(Token::RBRACE(info), '}'))},
                '"' => // assert empty buffer 
                    return Some(self.quoted_string()),
                '\n'  => {if !self.buf.is_empty() { let token = Some(self.word());self.nextline();return token;} else {self.nextline();}},
                ' ' | '\t' | '\r' => if !self.buf.is_empty() { return Some(self.word()); },
                _ => self.buf.push(c),
            }
        }
        if self.buf.is_empty() {
            None
        } else {
            Some(self.word())
        }
    }
}

pub trait TokenizerTrait<'a, I>: Sized
where
    I: Iterator<Item = char>,
{
    fn tokens(self: &'a mut Self) -> Tokenizer<'a>;
}

impl<'a, I: Iterator<Item = char>> TokenizerTrait<'a, I> for I {
    fn tokens(self: &'a mut Self) -> Tokenizer<'a> {
        Tokenizer {
            buf: Vec::new(),
            chars: self,
            line: 1,
            col: 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Token::*;
    use super::*;

    fn word(s: &str, line: usize, column: usize) -> Option<Token> {
        Some(WORD(TokenInfo { line, column }, s.to_owned()))
    }

    fn num(s: i32, line: usize, column: usize) -> Option<Token> {
        Some(NUM(TokenInfo { line, column }, s))
    }

    fn string(s: &str, line: usize, column: usize) -> Option<Token> {
        Some(STRING(TokenInfo { line, column }, s.to_owned()))
    }

    #[test]
    fn tokenizer_stream() {
        let input = r#"trim_line
truncate_by 12
insert "const KW_"
"#;

        let mut chars = input.chars();
        let mut tokens = chars.tokens();

        assert_eq!(word("trim_line", 1, 10), tokens.next());

        assert_eq!(word("truncate_by", 2, 12), tokens.next());

        assert_eq!(num(12, 2, 15), tokens.next());

        assert_eq!(word("insert", 3, 7), tokens.next());

        assert_eq!(string("const KW_", 3, 18), tokens.next());

        assert_eq!(None, tokens.next());
    }
}
