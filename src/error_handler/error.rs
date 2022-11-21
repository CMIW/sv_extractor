use thiserror::Error;

#[derive(Error, Debug)]
pub enum SVExtractorError{
	#[error("Invalid header, expected {expected:?} but found {found:?}")]
    InvalidHeader {
		expected: String,
        found: String,
	},

    #[error("Directory {path:?} doesn't exists")]
    NotDir {
        path: String,
    },

	/// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Represents all other cases of `std::num::TryFromIntError`.
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),

    /// Represents all other cases of `std::convert::Infallible`.
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),

    /// Represents all other cases of `serde_json::Error`.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}