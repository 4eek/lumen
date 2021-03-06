use std::convert::{TryFrom, TryInto};

use liblumen_alloc::erts::term::prelude::*;

pub struct MinorVersion(u8);

impl MinorVersion {
    const MIN_U8: u8 = 0;
    const MAX_U8: u8 = 2;
}

impl Default for MinorVersion {
    fn default() -> Self {
        Self(1)
    }
}

impl TryFrom<Term> for MinorVersion {
    type Error = TryFromTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        let term_u8: u8 = term.try_into()?;

        if Self::MIN_U8 <= term_u8 && term_u8 <= Self::MAX_U8 {
            Ok(Self(term_u8))
        } else {
            Err(TryFromTermError::OutOfRange)
        }
    }
}

pub enum TryFromTermError {
    OutOfRange,
    Type,
}

impl From<TryIntoIntegerError> for TryFromTermError {
    fn from(error: TryIntoIntegerError) -> Self {
        match error {
            TryIntoIntegerError::Type => TryFromTermError::Type,
            TryIntoIntegerError::OutOfRange => TryFromTermError::OutOfRange,
        }
    }
}
