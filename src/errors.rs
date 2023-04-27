use std::fmt::Debug;

/// Records a failed conversion.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("strconv.{func}: parsing '{num}': {err}")]
pub struct NumError {
    /// the failing function (parse_int, parse_uint)
    pub func: String,
    /// the input
    pub num: String,
    /// the reason the conversion failed
    #[source]
    pub err: NumErrorCause,
}

/// Reason of conversion failed.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum NumErrorCause {
    #[error("invalid base {0}")]
    InvalidBase(u8),
    #[error("invalid bit size {0}")]
    InvalidBitSize(u8),
    #[error("invalid syntax")]
    InvalidSyntax,
    /// Indicates that a signed value is out of range for the target type.
    #[error("signed value out of range: {bound_hint}")]
    OutOfRangeSigned { bound_hint: i64 },
    /// Indicates that a unsigned value is out of range for the target type.
    #[error("unsigned value out of range: {bound_hint}")]
    OutOfRangeUnsigned { bound_hint: u64 },
}

impl NumError {
    pub(crate) fn base<S, T>(func: S, s: T, b: u8) -> Self
    where
        S: ToString,
        T: ToString,
    {
        Self {
            func: func.to_string(),
            num: s.to_string(),
            err: NumErrorCause::InvalidBase(b),
        }
    }

    pub(crate) fn bit_size<S, T>(func: S, s: T, bit_size: u8) -> Self
    where
        S: ToString,
        T: ToString,
    {
        Self {
            func: func.to_string(),
            num: s.to_string(),
            err: NumErrorCause::InvalidBitSize(bit_size),
        }
    }

    pub(crate) fn range_signed<S, T>(func: S, s: T, bound_hint: i64) -> Self
    where
        S: ToString,
        T: ToString,
    {
        Self {
            func: func.to_string(),
            num: s.to_string(),
            err: NumErrorCause::OutOfRangeSigned { bound_hint },
        }
    }

    pub(crate) fn range_unsigned<S, T>(func: S, s: T, bound_hint: u64) -> Self
    where
        S: ToString,
        T: ToString,
    {
        Self {
            func: func.to_string(),
            num: s.to_string(),
            err: NumErrorCause::OutOfRangeUnsigned { bound_hint },
        }
    }

    pub(crate) fn syntax<S, T>(func: S, s: T) -> Self
    where
        S: ToString,
        T: ToString,
    {
        Self {
            func: func.to_string(),
            num: s.to_string(),
            err: NumErrorCause::InvalidSyntax,
        }
    }
}
