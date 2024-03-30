use std::io::Write;

use basic::{BasicKeyword, BasicLine, BasicProgram, BasicToken, PetsciiEncodingOptions, PetsciiString};
use serde::{ser, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub line_number_start: u16,

    pub line_number_increment: u16,

    pub encoding_options: PetsciiEncodingOptions,

    pub emit_bytes_length: bool,

    pub emit_sequence_length: bool,

    pub emit_map_length: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            line_number_start: 1000,
            line_number_increment: 1,
            encoding_options: Default::default(),
            emit_bytes_length: false,
            emit_sequence_length: false,
            emit_map_length: false,
        }
    }
}

pub struct Serializer {
    options: Options,
    basic_program: BasicProgram,
    basic_next_line: BasicLine,
    basic_next_line_number: u16,
    basic_next_line_started: bool,
}

pub fn to_writer<W, T>(mut writer: W, value: &T, options: Options) -> Result<()>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer {
        options,
        basic_program: BasicProgram {
            load_address: 0x0801,
            encoding_options: options.encoding_options,
            contents: vec![],
        },
        basic_next_line: BasicLine(options.line_number_start, vec![BasicKeyword::Data.into()]),
        basic_next_line_number: options.line_number_start,
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
            self.basic_program.contents.push(self.basic_next_line.clone());

            self.basic_next_line_number += self.options.line_number_increment; // TODO: Overflow
            self.basic_next_line = BasicLine(self.basic_next_line_number, vec![BasicKeyword::Data.into()]);
            self.basic_next_line_started = false;
        }
        Ok(())
    }

    fn quote_and_escape(&self, s: impl ToString) -> String {
        format!("\"{}\"", s.to_string().replace('"', "'"))
    }

    fn format_basic_data_item(&self, s: impl AsRef<str>) -> BasicToken {
        if self.basic_next_line_started {
            BasicToken::Raw(PetsciiString(format!(", {}", s.as_ref())))
        } else {
            BasicToken::Raw(PetsciiString(format!(" {}", s.as_ref())))
        }
    }

    fn emit_basic_data_item(&mut self, s: impl ToString) -> Result<()> {
        let token = self.format_basic_data_item(s.to_string());
        if let Err(_) = self.basic_next_line.push_token(token) {
            self.finalize_line()?;

            let token = self.format_basic_data_item(s.to_string());
            if let Err(_) = self.basic_next_line.push_token(token) {
                panic!("Failed to serialize token");
            }
        }

        self.basic_next_line_started = true;
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
        self.emit_basic_data_item(if v { "1" } else { "0" })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.emit_basic_data_item(v)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        let escaped = self.quote_and_escape(v);
        self.emit_basic_data_item(escaped)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let escaped = self.quote_and_escape(v);
        self.emit_basic_data_item(escaped)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        if self.options.emit_bytes_length {
            self.serialize_u64(v.len() as u64)?;
            self.finalize_line()?;
        }

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

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str) -> Result<()> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        variant_index.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if self.options.emit_sequence_length {
            self.serialize_u64(len.unwrap_or(0) as u64)?;
            self.finalize_line()?;
        }

        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if self.options.emit_map_length {
            self.serialize_u64(len.unwrap_or(0) as u64)?;
            self.finalize_line()?;
        }

        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
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

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<()>
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

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
