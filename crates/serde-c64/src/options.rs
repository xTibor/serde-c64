use basic::{PetsciiEncodingOptions, MAX_LINE_LENGTH};

#[derive(Debug, Copy, Clone)]
pub struct ContainerPrefixOptions {
    /// Prefix sequence-like types (`Vec`, `&[T]`, etc.) with their lengths.
    pub sequence_length: bool,

    /// Prefix hash map-like types (`HashMap`, etc.) with their length.
    pub map_length: bool,

    /// Prefix tuple-like types (`(T, T)`, `struct S(T, T)`, `&[T; N]`, etc.) with their length.
    pub tuple_length: bool,
}

impl Default for ContainerPrefixOptions {
    fn default() -> Self {
        Self {
            sequence_length: true,
            map_length: true,
            tuple_length: false,
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum StringQuotationMethod {
    /// Only quotes strings when necessary.
    ///
    /// Example:
    /// `1000 DATA AA, BB, CC, " DD", "EE "`
    #[default]
    WhenNecessary,

    /// Always quotes strings.
    ///
    /// Example:
    /// `1000 DATA "AA", "BB", "CC", " DD", "EE "`
    Always,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Copy, Clone)]
pub struct SpacingOptions {
    /// Put spaces between `DATA` statements and the first data entries.
    ///
    /// Example:
    /// * `false`: `1000 DATA1234`
    /// * `true`: `1000 DATA 1234`
    pub space_after_data_statement: bool,

    /// Put spaces between data entries.
    ///
    /// Example:
    /// * `false`: `1000 DATA 1,2,3,4`
    /// * `true`: `1000 DATA 1, 2, 3, 4`
    pub space_between_data_entries: bool,
}

impl Default for SpacingOptions {
    fn default() -> Self {
        Self {
            space_after_data_statement: true,
            space_between_data_entries: true,
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub line_length: usize,

    pub line_number_start: u16,

    pub line_number_increment: u16,

    pub encoding_options: PetsciiEncodingOptions,

    pub container_prefix_options: ContainerPrefixOptions,

    pub spacing_options: SpacingOptions,

    pub string_quotation_method: StringQuotationMethod,

    pub emit_enum_names: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            line_length: MAX_LINE_LENGTH,
            line_number_start: 1000,
            line_number_increment: 1,
            encoding_options: PetsciiEncodingOptions::default(),
            container_prefix_options: ContainerPrefixOptions::default(),
            spacing_options: SpacingOptions::default(),
            string_quotation_method: StringQuotationMethod::default(),
            emit_enum_names: false,
        }
    }
}
