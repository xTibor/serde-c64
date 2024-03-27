use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct PetsciiEncodingOptions {
    pub variant: PetsciiVariant,
}

#[derive(Debug)]
pub enum PetsciiVariant {
    Unshifted,
    Shifted,
}

#[derive(Debug, Clone)]
pub struct PetsciiString(pub String);

impl From<&str> for PetsciiString {
    fn from(string: &str) -> Self {
        Self(string.to_string())
    }
}

lazy_static! {
    #[rustfmt::skip]
    static ref PETSCII_UNSHIFTED_MAP: HashMap<char, u8> = HashMap::from([
        (' ',  0x20), ('!',  0x21), ('"',  0x22), ('#',  0x23), ('$',  0x24), ('%',  0x25), ('&',  0x26), ('\'', 0x27),
        ('(',  0x28), (')',  0x29), ('*',  0x2A), ('+',  0x2B), (',',  0x2C), ('-',  0x2D), ('.',  0x2E), ('/',  0x2F),
        ('0',  0x30), ('1',  0x31), ('2',  0x32), ('3',  0x33), ('4',  0x34), ('5',  0x35), ('6',  0x36), ('7',  0x37),
        ('8',  0x38), ('9',  0x39), (':',  0x3A), (';',  0x3B), ('<',  0x3C), ('=',  0x3D), ('>',  0x3E), ('?',  0x3F),
        ('@',  0x40), ('A',  0x41), ('B',  0x42), ('C',  0x43), ('D',  0x44), ('E',  0x45), ('F',  0x46), ('G',  0x47),
        ('H',  0x48), ('I',  0x49), ('J',  0x4A), ('K',  0x4B), ('L',  0x4C), ('M',  0x4D), ('N',  0x4E), ('O',  0x4F),
        ('P',  0x50), ('Q',  0x51), ('R',  0x52), ('S',  0x53), ('T',  0x54), ('U',  0x55), ('V',  0x56), ('W',  0x57),
        ('X',  0x58), ('Y',  0x59), ('Z',  0x5A), ('[',  0x5B), ('£',  0x5C), (']',  0x5D), ('↑',  0x5E), ('←',  0x5F),
                      ('a',  0x41), ('b',  0x42), ('c',  0x43), ('d',  0x44), ('e',  0x45), ('f',  0x46), ('g',  0x47),
        ('h',  0x48), ('i',  0x49), ('j',  0x4A), ('k',  0x4B), ('l',  0x4C), ('m',  0x4D), ('n',  0x4E), ('o',  0x4F),
        ('p',  0x50), ('q',  0x51), ('r',  0x52), ('s',  0x53), ('t',  0x54), ('u',  0x55), ('v',  0x56), ('w',  0x57),
        ('x',  0x58), ('y',  0x59), ('z',  0x5A),
        ('♠',  0x61), ('♥',  0x73), ('♣',  0x78), ('♦',  0x7A), ('π',  0x7e),
    ]);

    #[rustfmt::skip]
    static ref PETSCII_SHIFTED_MAP: HashMap<char, u8> = HashMap::from([
        (' ',  0x20), ('!',  0x21), ('"',  0x22), ('#',  0x23), ('$',  0x24), ('%',  0x25), ('&',  0x26), ('\'', 0x27),
        ('(',  0x28), (')',  0x29), ('*',  0x2A), ('+',  0x2B), (',',  0x2C), ('-',  0x2D), ('.',  0x2E), ('/',  0x2F),
        ('0',  0x30), ('1',  0x31), ('2',  0x32), ('3',  0x33), ('4',  0x34), ('5',  0x35), ('6',  0x36), ('7',  0x37),
        ('8',  0x38), ('9',  0x39), (':',  0x3A), (';',  0x3B), ('<',  0x3C), ('=',  0x3D), ('>',  0x3E), ('?',  0x3F),
        ('@',  0x40), ('a',  0x41), ('b',  0x42), ('c',  0x43), ('d',  0x44), ('e',  0x45), ('f',  0x46), ('g',  0x47),
        ('h',  0x48), ('i',  0x49), ('j',  0x4A), ('k',  0x4B), ('l',  0x4C), ('m',  0x4D), ('n',  0x4E), ('o',  0x4F),
        ('p',  0x50), ('q',  0x51), ('r',  0x52), ('s',  0x53), ('t',  0x54), ('u',  0x55), ('v',  0x56), ('w',  0x57),
        ('x',  0x58), ('y',  0x59), ('z',  0x5A), ('[',  0x5B), ('£',  0x5C), (']',  0x5D), ('↑',  0x5E), ('←',  0x5F),
                      ('A',  0xC1), ('B',  0xC2), ('C',  0xC3), ('D',  0xC4), ('E',  0xC5), ('F',  0xC6), ('G',  0xC7),
        ('H',  0xC8), ('I',  0xC9), ('J',  0xCA), ('K',  0xCB), ('L',  0xCC), ('M',  0xCD), ('N',  0xCE), ('O',  0xCF),
        ('P',  0xD0), ('Q',  0xD1), ('R',  0xD2), ('S',  0xD3), ('T',  0xD4), ('U',  0xD5), ('V',  0xD6), ('W',  0xD7),
        ('X',  0xD8), ('Y',  0xD9), ('Z',  0xDA),
    ]);
}

impl PetsciiString {
    pub fn to_petscii(&self, encoding_options: &PetsciiEncodingOptions) -> Vec<u8> {
        let petscii_map: &HashMap<_, _> = match encoding_options.variant {
            PetsciiVariant::Unshifted => &PETSCII_UNSHIFTED_MAP,
            PetsciiVariant::Shifted => &PETSCII_SHIFTED_MAP,
        };

        self.0
            .chars()
            .map(|c| *petscii_map.get(&c).unwrap_or(&0x3F))
            .collect::<Vec<u8>>()
    }
}
