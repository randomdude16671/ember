use crate::asm::tokens::*;
use crate::map;
use hashbrown::HashMap;

struct Rpos {
    ln: usize,
    col: usize,
}

impl Rpos {
    fn new() -> Self {
        Self { ln: 1, col: 0 }
    }

    fn advance(&mut self, char: Option<char>) {
        if let Some(c) = char {
            if c == '\n' {
                self.ln += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }
}

pub struct Tokenizer<'a> {
    src: &'a String,
    input: std::str::Chars<'a>,
    ch: Option<char>,
    file_path: String,
    position: usize, // current byte offset
    read_pos: usize, // next byte offset
    pos: Rpos,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a String, file_path: String) -> Self {
        let mut input = src.chars();
        let ch = input.next();
        let keywords: HashMap<String, TokenType> = map! {
            "push" => TokenType::Instruction(Instruction::Push),
            "pop" => TokenType::Instruction(Instruction::Pop),
            "dup" => TokenType::Instruction(Instruction::Dup),
            "swap" => TokenType::Instruction(Instruction::Swap),
            "add" => TokenType::Instruction(Instruction::Add),
            "sub" => TokenType::Instruction(Instruction::Sub),
            "mul" => TokenType::Instruction(Instruction::Mul),
            "div" => TokenType::Instruction(Instruction::Div),
            "jmp" => TokenType::Instruction(Instruction::Jmp),
            "jz" => TokenType::Instruction(Instruction::Jz),
            "jnz" => TokenType::Instruction(Instruction::Jnz),
            "call" => TokenType::Instruction(Instruction::Call),
            "trap" => TokenType::Instruction(Instruction::Trap),
        };
        Self {
            src,
            input,
            ch,
            file_path,
            pos: Rpos::new(),
            read_pos: ch.map_or(0, |c| c.len_utf8()),
            position: 0,
            keywords,
        }
    }

    pub fn read_char(&mut self) {
        self.ch = self.input.next();

        if let Some(ch) = self.ch {
            self.position = self.read_pos;
            self.read_pos += ch.len_utf8(); // advance by how much the utf8 length of char is
        } else {
            // EOF
            self.position = self.read_pos;
        }

        self.pos.advance(self.ch);
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.input.clone().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_whitespace() && c != '\n' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.ch {
            if c.is_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        let literal = self.src[start..self.position].to_string();
        if self.keywords.contains_key(&literal) {
            let typ = self.keywords.get(&literal).unwrap();
            return Token {
                typ: typ.clone(),
                literal: literal.clone(),
            };
        }
        let typ = self.keywords.get(&literal).cloned().unwrap_or_else(|| {
            TokenType::Label(Identifier {
                name: literal.clone(),
            })
        });
        Token { typ, literal }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.ch {
            if c.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }
        let literal = self.src[start..self.position].to_string();
        let num = literal.parse::<i64>().unwrap();
        Token {
            typ: TokenType::Number(Literal::Number(num)),
            literal,
        }
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // skip opening "
        let start = self.position;
        while let Some(c) = self.ch {
            if c == '"' {
                break;
            }
            self.read_char();
        }
        let literal = self.src[start..self.position].to_string();
        self.read_char(); // skip closing "
        Token {
            typ: TokenType::String(Literal::String(literal.clone())),
            literal,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.ch {
            Some(c) => {
                let tok = match c {
                    'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                    '0'..='9' => self.read_number(),
                    '"' => self.read_string(),
                    ':' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Delimiter(Delimiter::Colon),
                            literal: ":".to_string(),
                        }
                    }
                    ',' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Delimiter(Delimiter::Comma),
                            literal: ",".to_string(),
                        }
                    }
                    '+' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Operator(Op::Plus),
                            literal: "+".to_string(),
                        }
                    }
                    '-' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Operator(Op::Minus),
                            literal: "-".to_string(),
                        }
                    }
                    '*' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Operator(Op::Multiply),
                            literal: "*".to_string(),
                        }
                    }
                    '/' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Operator(Op::Divide),
                            literal: "/".to_string(),
                        }
                    }
                    '\n' => {
                        self.read_char();
                        Token {
                            typ: TokenType::Special(Special::Eol),
                            literal: "\n".to_string(),
                        }
                    }
                    _ => {
                        self.read_char();
                        // Unknown token, perhaps panic or return error token
                        Token {
                            typ: TokenType::Special(Special::Eof),
                            literal: c.to_string(),
                        }
                    }
                };
                Some(tok)
            }
            None => None,
        }
    }
}
