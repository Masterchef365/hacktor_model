use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

/// Unique data type ID for dynamic typing using AnonymousData
pub type DataTypeID = u64;

/// Anonymous data type, can be nested.
#[derive(Clone, Serialize, Deserialize)]
pub struct AnonymousData {
    type_id: DataTypeID,
    data: Box<[u8]>,
}

/// Trait for data types that can be serialized for Anonymous transfer.
pub trait HasTypeID {
    /// Unique ID for the data type (I use `capnp id` to generate this)
    const TYPE_ID: DataTypeID;
}

// TODO: Use stdlib traits for these...
impl AnonymousData {
    pub fn into_type<T: HasTypeID + Serialize>(object: &T) -> Result<Self, bincode::Error> {
        Ok(Self {
            type_id: T::TYPE_ID,
            data: bincode::serialize(&object)?.into_boxed_slice(),
        })
    }

    pub fn as_type<'a, T: HasTypeID + Deserialize<'a>>(&'a self) -> Result<T, AnonError> {
        if T::TYPE_ID != self.type_id {
            return Err(AnonError::TypeIDMismatch {
                expected: T::TYPE_ID,
                got: self.type_id,
            });
        }
        bincode::deserialize(&self.data).map_err(AnonError::Bincode)
    }
}

impl HasTypeID for AnonymousData {
    const TYPE_ID: DataTypeID = 0xfc9463e33a98c637;
}

/// Error encountered during anonymous conversion
#[derive(Debug)]
pub enum AnonError {
    Bincode(bincode::Error),
    TypeIDMismatch {
        expected: DataTypeID,
        got: DataTypeID,
    },
}

impl Display for AnonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnonError::Bincode(e) => Display::fmt(e, f),
            AnonError::TypeIDMismatch { expected, got } => {
                write!(f, "Expected type ID 0x{:X}, got 0x{:X}", expected, got)
            }
        }
    }
}

impl std::error::Error for AnonError {}

impl Debug for AnonymousData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AnonymousData 0x{:X} ({} bytes)",
            self.type_id,
            self.data.len()
        )
    }
}
