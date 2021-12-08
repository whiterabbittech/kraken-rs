pub use std::fmt;
pub use std::marker::PhantomData;
pub use super::error_wrapper::*;

pub type AskError = ParseError<AskInfoMetadata>;
pub type BidError = ParseError<BidInfoMetadata>;
pub type HighError = ParseError<HighInfoMetadata>;
pub type LowError = ParseError<LowInfoMetadata>;
pub type LastTradeError = ParseError<LastTradeInfoMetadata>;

pub enum ParseError<T: ErrorWrapper> {
    TryFrom(PhantomData<T>),
    NoKey(PhantomData<T>),
    NoneValue(PhantomData<T>),
    NotAString(PhantomData<T>),
    NotAFloat(PhantomData<T>),
}

impl<T: ErrorWrapper> ParseError<T> {
    pub fn try_from_error() -> Self {
        Self::TryFrom(PhantomData)
    }

    pub fn no_key_error() -> Self {
        Self::NoKey(PhantomData)
    }

    pub fn none_value_error() -> Self {
        Self::NoneValue(PhantomData)
    }

    pub fn not_a_string_error() -> Self {
        Self::NotAString(PhantomData)
    }

    pub fn not_a_float_error() -> Self {
        Self::NotAFloat(PhantomData)
    }
}

impl<T: ErrorWrapper> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFrom(_) => {
                let wrapper = T::try_failure_wrapper();
                write!(f, "{}: Value is not an Object.", wrapper)
            }
            Self::NoKey(_) => {
                let wrapper = T::no_key_wrapper();
                let key = T::on_no_key();
                write!(f, "{}: Object no key \"{}\"", wrapper, key)
            }
            Self::NoneValue(_) => {
                let wrapper = T::array_none_wrapper();
                write!(f, "{}: Value at index provided is None", wrapper)
            }
            Self::NotAString(_) => {
                let wrapper = T::not_a_string_wrapper();
                write!(f, "{}: Value is not a String", wrapper)
            }
            Self::NotAFloat(_) => {
                let wrapper = T::not_a_float_wrapper();
                write!(f, "{}: String at array index is not a Number", wrapper)
            }
        }
    }
}

impl<T: ErrorWrapper> fmt::Debug for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
