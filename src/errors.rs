use error_context::*;
use snafu::prelude::*;

#[derive(Debug, Snafu, Clone)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Error: {}", msg))]
    GenericError { msg: String },
    #[snafu(display("Format conversion error: {}", msg))]
    FormatConversionError { msg: String },
    #[snafu(display("Field not found: {}", msg))]
    FieldNotFound { msg: String },
}
error_wrap!(std::num::ParseFloatError, Error::FormatConversionError);
error_wrap!(std::num::ParseIntError, Error::FormatConversionError);
error_wrap!(std::str::ParseBoolError, Error::FormatConversionError);
