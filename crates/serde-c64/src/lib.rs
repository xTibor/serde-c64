mod error;
mod options;
mod ser;

pub use error::{Error, Result};
pub use options::{ContainerPrefixOptions, Options, StringQuotationMethod};
pub use ser::{to_writer, Serializer};
