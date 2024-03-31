use basic::{PetsciiEncodingOptions, MAX_LINE_LENGTH};

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub line_length: usize,

    pub line_number_start: u16,

    pub line_number_increment: u16,

    pub encoding_options: PetsciiEncodingOptions,

    pub emit_bytes_length: bool,

    pub emit_sequence_length: bool,

    pub emit_map_length: bool,

    pub emit_enum_names: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            line_length: MAX_LINE_LENGTH,
            line_number_start: 1000,
            line_number_increment: 1,
            encoding_options: Default::default(),
            emit_bytes_length: false,
            emit_sequence_length: false,
            emit_map_length: false,
            emit_enum_names: false,
        }
    }
}
