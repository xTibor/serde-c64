use crate::petscii::{PetsciiEncodingOptions, PetsciiString};

#[derive(Debug, Clone)]
pub enum BasicToken {
    Keyword(BasicKeyword),
    Raw(PetsciiString),
}

#[allow(unused)]
#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum BasicKeyword {
    End       = 0x80, For       = 0x81, Next      = 0x82, Data      = 0x83,
    InputHash = 0x84, Input     = 0x85, Dim       = 0x86, Read      = 0x87,
    Let       = 0x88, Goto      = 0x89, Run       = 0x8A, If        = 0x8B,
    Restore   = 0x8C, Gosub     = 0x8D, Return    = 0x8E, Rem       = 0x8F,
    Stop      = 0x90, On        = 0x91, Wait      = 0x92, Load      = 0x93,
    Save      = 0x94, Verify    = 0x95, Def       = 0x96, Poke      = 0x97,
    PrintHash = 0x98, Print     = 0x99, Cont      = 0x9A, List      = 0x9B,
    Clr       = 0x9C, Cmd       = 0x9D, Sys       = 0x9E, Open      = 0x9F,
    Close     = 0xA0, Get       = 0xA1, New       = 0xA2, Tab       = 0xA3,
    To        = 0xA4, Fn        = 0xA5, Spc       = 0xA6, Then      = 0xA7,
    Not       = 0xA8, Step      = 0xA9, OpAdd     = 0xAA, OpSub     = 0xAB,
    OpMul     = 0xAC, OpDiv     = 0xAD, OpPow     = 0xAE, And       = 0xAF,
    Or        = 0xB0, OpGreater = 0xB1, OpEquals  = 0xB2, OpLess    = 0xB3,
    Sgn       = 0xB4, Int       = 0xB5, Abs       = 0xB6, Usr       = 0xB7,
    Fre       = 0xB8, Pos       = 0xB9, Sqr       = 0xBA, Rnd       = 0xBB,
    Log       = 0xBC, Exp       = 0xBD, Cos       = 0xBE, Sin       = 0xBF,
    Tan       = 0xC0, Atn       = 0xC1, Peek      = 0xC2, Len       = 0xC3,
    Str       = 0xC4, Val       = 0xC5, Asc       = 0xC6, Chr       = 0xC7,
    Left      = 0xC8, Right     = 0xC9, Mid       = 0xCA, Go        = 0xCB,
}

impl BasicToken {
    pub fn to_bytes(&self, encoding_options: &PetsciiEncodingOptions) -> Vec<u8> {
        match self {
            BasicToken::Keyword(keyword) => vec![*keyword as u8],
            BasicToken::Raw(string) => string.to_petscii(encoding_options),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            BasicToken::Keyword(_) => 1,
            BasicToken::Raw(PetsciiString(string)) => string.chars().count(),
        }
    }
}

impl From<&str> for BasicToken {
    fn from(string: &str) -> Self {
        BasicToken::Raw(PetsciiString(string.to_owned()))
    }
}

impl From<BasicKeyword> for BasicToken {
    fn from(keyword: BasicKeyword) -> Self {
        BasicToken::Keyword(keyword)
    }
}
