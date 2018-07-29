
#[derive(Debug, PartialEq)]
pub enum Token {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    WORD(String),
    NUM(i32),
    STRING(String),
    QUOTE,
}

pub struct Tokenizer<'a> {
    buf: Vec<char>,
    chars: &'a mut Iterator<Item = char>,
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
        Token::NUM(num)
    } else {
        Token::WORD(result)
    }
}

fn quoted_string(tokenizer: &mut Tokenizer) -> Token {
    let mut escape = false;
    while let Some(c) = tokenizer.chars.next() {
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
            _ => tokenizer.buf.push(c),
        }
    }
    let result: String = tokenizer.buf.drain(..).collect();
    Token::STRING(result)
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            match c {
                '(' => return Some(word_or_token(self,Token::LPAREN, '(')),
                ')' => return Some(word_or_token(self,Token::LPAREN, ')')),
                '{' => return Some(word_or_token(self,Token::LBRACE, '{')),
                '}' => return Some(word_or_token(self,Token::RBRACE, '}')),
                '"' => // assert empty buffer 
                    return Some(quoted_string(self)),
                ' ' | '\t' | '\n' | '\r' => if !self.buf.is_empty() { return Some(word(self)); },
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
        }
    }
}
