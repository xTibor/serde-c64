use crate::petscii::PetsciiEncodingOptions;
use crate::token::BasicToken;

#[derive(Debug, Clone)]
pub struct BasicLine(pub u16, pub Vec<BasicToken>);

impl BasicLine {
    pub fn size(&self) -> usize {
        self.1.iter().map(BasicToken::size).sum()
    }

    pub fn push_token(&mut self, token: BasicToken) -> Result<(), &str> {
        if token.size() + self.size() <= 250 {
            self.1.push(token);
            Ok(())
        } else {
            Err("Line overflow")
        }
    }
}

#[derive(Debug)]
pub struct BasicProgram {
    pub load_address: u16,
    pub encoding_options: PetsciiEncodingOptions,
    pub contents: Vec<BasicLine>,
}

impl BasicProgram {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut prg_bin = vec![];
        prg_bin.extend(self.load_address.to_le_bytes());

        let mut next_line_address = self.load_address;

        for BasicLine(line_number, line_tokens) in &self.contents {
            let mut line_bin = vec![];
            line_bin.extend(line_number.to_le_bytes());
            line_bin.extend(line_tokens.iter().flat_map(|t| t.to_bytes(&self.encoding_options)));
            line_bin.push(0x00);

            next_line_address += line_bin.len() as u16 + 0x02;

            prg_bin.extend(next_line_address.to_le_bytes());
            prg_bin.extend(line_bin);
        }

        prg_bin.extend(0x0000u16.to_le_bytes());
        prg_bin
    }
}

#[cfg(test)]
mod tests {
    use crate::petscii::{PetsciiEncodingOptions, PetsciiVariant};
    use crate::program::{BasicLine, BasicProgram};
    use crate::token::BasicKeyword;

    #[test]
    pub fn test_data() {
        let basic_program = BasicProgram {
            load_address: 0x0801,
            encoding_options: PetsciiEncodingOptions {
                variant: PetsciiVariant::Unshifted,
            },
            #[rustfmt::skip]
            contents: vec![
                BasicLine(10, vec![BasicKeyword::Data.into(), " 1,2,3,4".into()]),
                BasicLine(20, vec![BasicKeyword::Data.into(), " 5,6,7,8".into()]),
                BasicLine(30, vec![BasicKeyword::Data.into(), " 1,1,1,1".into()]),
                BasicLine(40, vec![BasicKeyword::Data.into(), " 2,2,2,2".into()]),
                BasicLine(50, vec![BasicKeyword::Data.into(), " 3,3,3,3".into()]),
                BasicLine(60, vec![BasicKeyword::Data.into(), " 4,4,4,4".into()]),
            ],
        };

        assert_eq!(
            basic_program.to_bytes(),
            vec![
                0x01, 0x08, 0x0F, 0x08, 0x0A, 0x00, 0x83, 0x20, 0x31, 0x2C, 0x32, 0x2C, 0x33, 0x2C, 0x34, 0x00, 0x1D,
                0x08, 0x14, 0x00, 0x83, 0x20, 0x35, 0x2C, 0x36, 0x2C, 0x37, 0x2C, 0x38, 0x00, 0x2B, 0x08, 0x1E, 0x00,
                0x83, 0x20, 0x31, 0x2C, 0x31, 0x2C, 0x31, 0x2C, 0x31, 0x00, 0x39, 0x08, 0x28, 0x00, 0x83, 0x20, 0x32,
                0x2C, 0x32, 0x2C, 0x32, 0x2C, 0x32, 0x00, 0x47, 0x08, 0x32, 0x00, 0x83, 0x20, 0x33, 0x2C, 0x33, 0x2C,
                0x33, 0x2C, 0x33, 0x00, 0x55, 0x08, 0x3C, 0x00, 0x83, 0x20, 0x34, 0x2C, 0x34, 0x2C, 0x34, 0x2C, 0x34,
                0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    pub fn test_petscii_unshifted() {
        let basic_program = BasicProgram {
            load_address: 0x0801,
            encoding_options: PetsciiEncodingOptions {
                variant: PetsciiVariant::Unshifted,
            },
            #[rustfmt::skip]
            contents: vec![
                BasicLine(10, vec![BasicKeyword::Rem.into(), " 0123456789".into()]),
                BasicLine(20, vec![BasicKeyword::Rem.into(), " ABCD EFGH IJKL MNOP QRST UVWX YZ".into()]),
            ],
        };

        assert_eq!(
            basic_program.to_bytes(),
            vec![
                0x01, 0x08, 0x12, 0x08, 0x0A, 0x00, 0x8F, 0x20, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                0x39, 0x00, 0x39, 0x08, 0x14, 0x00, 0x8F, 0x20, 0x41, 0x42, 0x43, 0x44, 0x20, 0x45, 0x46, 0x47, 0x48,
                0x20, 0x49, 0x4A, 0x4B, 0x4C, 0x20, 0x4D, 0x4E, 0x4F, 0x50, 0x20, 0x51, 0x52, 0x53, 0x54, 0x20, 0x55,
                0x56, 0x57, 0x58, 0x20, 0x59, 0x5A, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    pub fn test_petscii_shifted() {
        let basic_program = BasicProgram {
            load_address: 0x0801,
            encoding_options: PetsciiEncodingOptions {
                variant: PetsciiVariant::Shifted,
            },
            #[rustfmt::skip]
            contents: vec![
                BasicLine(10, vec![BasicKeyword::Rem.into(), " \"0123456789\"".into()]),
                BasicLine(20, vec![BasicKeyword::Rem.into(), " \"ABCD EFGH IJKL MNOP QRST UVWX YZ\"".into()]),
                BasicLine(30, vec![BasicKeyword::Rem.into(), " \"abcd efgh ijkl mnop qrst uvwx yz\"".into()]),
            ],
        };

        assert_eq!(
            basic_program.to_bytes(),
            vec![
                0x01, 0x08, 0x14, 0x08, 0x0A, 0x00, 0x8F, 0x20, 0x22, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
                0x38, 0x39, 0x22, 0x00, 0x3D, 0x08, 0x14, 0x00, 0x8F, 0x20, 0x22, 0xC1, 0xC2, 0xC3, 0xC4, 0x20, 0xC5,
                0xC6, 0xC7, 0xC8, 0x20, 0xC9, 0xCA, 0xCB, 0xCC, 0x20, 0xCD, 0xCE, 0xCF, 0xD0, 0x20, 0xD1, 0xD2, 0xD3,
                0xD4, 0x20, 0xD5, 0xD6, 0xD7, 0xD8, 0x20, 0xD9, 0xDA, 0x22, 0x00, 0x66, 0x08, 0x1E, 0x00, 0x8F, 0x20,
                0x22, 0x41, 0x42, 0x43, 0x44, 0x20, 0x45, 0x46, 0x47, 0x48, 0x20, 0x49, 0x4A, 0x4B, 0x4C, 0x20, 0x4D,
                0x4E, 0x4F, 0x50, 0x20, 0x51, 0x52, 0x53, 0x54, 0x20, 0x55, 0x56, 0x57, 0x58, 0x20, 0x59, 0x5A, 0x22,
                0x00, 0x00, 0x00
            ]
        );
    }
}
