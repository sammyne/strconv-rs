use std::fmt::Debug;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("strconv.{func}: parsing '{num}': {err}")]
pub struct NumError {
    pub func: String,
    pub num: String,
    #[source]
    pub err: NumErrorCause,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum NumErrorCause {
    #[error("invalid base {0}")]
    Base(u8),
    #[error("invalid bit size {0}")]
    BitSize(u8),
    #[error("signed value out of range: {bound_hint}")]
    RangeSigned { bound_hint: i64 },
    #[error("unsigned value out of range: {bound_hint}")]
    RangeUnsigned { bound_hint: u64 },
    #[error("invalid syntax")]
    Syntax,
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
            err: NumErrorCause::Base(b),
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
            err: NumErrorCause::BitSize(bit_size),
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
            err: NumErrorCause::RangeSigned { bound_hint },
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
            err: NumErrorCause::RangeUnsigned { bound_hint },
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
            err: NumErrorCause::Syntax,
        }
    }
}
