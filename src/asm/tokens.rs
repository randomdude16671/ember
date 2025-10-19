#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Push,
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Jmp,
    Jz,
    Jnz,
    Call,
    Trap,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(i64),
}

#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    StrLen,
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    Comma,
    Colon,
}

#[derive(Debug, Clone)]
pub enum Special {
    Eol,
    Eof,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Label(Identifier),
    Instruction(Instruction),
    Register(Identifier),
    Delimiter(Delimiter),
    String(Literal),
    Number(Literal),
    Operator(Op),
    Special(Special),
}
