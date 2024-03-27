use basic::{BasicKeyword, BasicLine, BasicProgram, BasicToken, PetsciiEncodingOptions, PetsciiString};
use serde::{ser, Serialize};
use std::io::Write;

use crate::error::{Error, Result};

pub struct Serializer {
    line_number_next: usize,
    line_number_increment: usize,
    basic_program: BasicProgram,

    basic_next_line: BasicLine,
    basic_next_line_started: bool,
}

pub fn to_writer<W, T>(mut writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer {
        line_number_next: 1000,
        line_number_increment: 1,
        basic_program: BasicProgram {
            load_address: 0x0801,
            encoding_options: PetsciiEncodingOptions {
                variant: basic::PetsciiVariant::Unshifted,
            },
            contents: vec![],
        },
        basic_next_line: BasicLine(0, vec![BasicKeyword::Data.into()]),
        basic_next_line_started: false,
    };

    value.serialize(&mut serializer)?;
    serializer.finalize_line()?;

    writer.write_all(&serializer.basic_program.to_bytes()).unwrap();

    Ok(())
}

impl Serializer {
    fn finalize_line(&mut self) -> Result<()> {
        if self.basic_next_line_started {
            self.basic_next_line.0 = u16::try_from(self.line_number_next).unwrap();
            self.basic_program.contents.push(self.basic_next_line.clone());

            self.line_number_next += self.line_number_increment;
            self.basic_next_line = BasicLine(0, vec![BasicKeyword::Data.into()]);
            self.basic_next_line_started = false;
        }
        Ok(())
    }

    fn push_str(&mut self, s: &str) -> Result<()> {
        if self.basic_next_line_started {
            self.push_token(BasicToken::Raw(PetsciiString(format!(", {}", s))))?;
        } else {
            self.push_token(BasicToken::Raw(PetsciiString(format!(" {}", s))))?;
        }
        Ok(())
    }

    fn push_token(&mut self, token: BasicToken) -> Result<()> {
        if let Err(_) = self.basic_next_line.push_token(token.clone()) {
            self.finalize_line()?;
            if let Err(_) = self.basic_next_line.push_token(token) {
                panic!("Failed to serialize token")
            } else {
                self.basic_next_line_started = true;
            }
        } else {
            self.basic_next_line_started = true;
        }

        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.push_str(if v { "1" } else { "0" })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.push_str(format!("{}", v).as_str())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // TODO: Escape quotes
        self.push_str(format!("\"{}\"", v).as_str())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        // TODO: Escape quotes
        self.push_str(format!("\"{}\"", v).as_str())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        for b in v {
            self.serialize_u8(*b)?;
        }
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_bool(false)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.serialize_bool(true)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<()> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        variant_index.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.finalize_line()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.finalize_line()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.finalize_line()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.finalize_line()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
