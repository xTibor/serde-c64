mod error;
mod options;
mod ser;

pub use error::{Error, Result};
pub use options::Options;
pub use ser::{to_writer, Serializer};
