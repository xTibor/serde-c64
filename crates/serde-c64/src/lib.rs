mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_writer, Options, Serializer};
