use thiserror::Error;

#[derive(Error, Debug)]
pub enum SVExtractorError{
	#[error("Invalid header, expected {expected:?} but found {found:?}")]
    InvalidHeader {
		expected: String,
        found: String,
	},

    #[error("Directory {path:?} doesn't exists")]
    NoDirectory {
        path: String,
    },

    #[error("{path:?} is a directory, expected a file")]
    IsADirectory {
        path: String,
    },

    #[error("Wrong extension, expected {expected:?} but found {found:?}")]
    WrongExtension{
        expected: String,
        found: String,
    },
    
    #[error("Failed to decompress the data")]
    OodleDecompressError,

	/// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Represents all other cases of `std::num::TryFromIntError`.
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),

    /// Represents all other cases of `std::convert::Infallible`.
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),

    /// Represents all other cases of `flatbuffers::InvalidFlatbuffer`.
    #[error(transparent)]
    InvalidFlatbuffer(#[from] flatbuffers::InvalidFlatbuffer),
}