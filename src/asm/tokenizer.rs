use hashbrown::HashMap;

#[derive(Debug)]
pub enum TokenType {
    LABEL,
    INSTRUCTION,
    REGISTER,
    COMMA,
    LITERAL,
    COMMENT,
    OPERATOR,
    EOL,
    EOF,
}

#[derive(Debug)]
struct Rpos {
    ln: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Token {
    typ: TokenType,
    literal: String,
}

pub struct Tokenizer<'a> {
    src: &'a str,
    input: std::str::Chars<'a>,
    ch: Option<char>,
    file_path: String,
    position: usize, // current byte offset
    read_pos: usize, // next byte offset
    pos: Rpos,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str, file_path: String) -> Self {
        let mut input = src.chars();
        let ch = input.next();
        Self {
            src,
            input,
            ch,
            file_path,
            pos: Rpos { ln: 1, col: 0 },
            read_pos: ch.map_or(0, |c| c.len_utf8()),
            position: 0,
            keywords: HashMap::new(),
        }
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.ch {
            self.position = self.read_pos;
            self.read_pos += c.len_utf8(); // advance by UTF-8 byte length
            self.ch = self.input.next();

            if c == '\n' {
                self.pos.ln += 1;
                self.pos.col = 0;
            } else {
                self.pos.col += 1;
            }
        } else {
            self.position = self.read_pos;
        }
    }
}
