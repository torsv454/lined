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

impl <'a> Tokenizer<'a> {
    fn info(&self) -> TokenInfo {
        TokenInfo {
            line: self.line,
            column: self.col,
        }
    }
}

fn word_or_token(tokenizer: &mut Tokenizer, token: Token, c: char) -> Token {
    if tokenizer.buf.is_empty() {
        token
    } else {
        let result = word(tokenizer);
        tokenizer.buf.push(c);
        result
    }
}

fn word(tokenizer: &mut Tokenizer) -> Token {
    let result: String = tokenizer.buf.drain(..).collect();
    if let Ok(num) = result.parse() {
        Token::NUM(tokenizer.info(), num)
    } else {
        Token::WORD(tokenizer.info(), result)
    }
}

fn quoted_string(tokenizer: &mut Tokenizer) -> Token {
    let mut escape = false;
    while let Some(c) = tokenizer.chars.next() {
        tokenizer.col += 1;
        // println!("c = {}, escape = {}", c, escape);
        match c {
            '"' if !escape => break,
            '"' => {
                escape = false;
                tokenizer.buf.push(c)
            }
            '\\' if !escape => escape = true,
            '\\' => {
                escape = false;
                tokenizer.buf.push(c)
            }
            '\n' => {
                tokenizer.line += 1;
                tokenizer.col = 0;
                tokenizer.buf.push(c);
            }
            _ => tokenizer.buf.push(c),
        }
    }
    let result: String = tokenizer.buf.drain(..).collect();
    Token::STRING(tokenizer.info(), result)
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            self.col += 1;

            match c {
                '(' => return { let info = self.info(); Some(word_or_token(self,Token::LPAREN(info), '('))},
                ')' => return  { let info = self.info(); Some(word_or_token(self,Token::LPAREN(info), ')'))},
                '{' => return  { let info = self.info(); Some(word_or_token(self,Token::LBRACE(info), '{'))},
                '}' => return  { let info = self.info(); Some(word_or_token(self,Token::RBRACE(info), '}'))},
                '"' => // assert empty buffer 
                    return Some(quoted_string(self)),
                ' ' | '\t' | '\n' | '\r' => {self.line += 1; self.col = 0; if !self.buf.is_empty() { return Some(word(self)); }},
                _ => self.buf.push(c),
            }
        }
        if self.buf.is_empty() {
            None
        } else {
            Some(word(self))
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
