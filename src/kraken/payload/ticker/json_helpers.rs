use std::fmt;
use std::marker::PhantomData;

pub enum ParseError<T: ErrorMetadata> {
    TryFromError(PhantomData<T>),
    NoKeyError(PhantomData<T>),
    NoneValueError(PhantomData<T>),
    NotAStringError(PhantomData<T>),
    NotAFloatError(PhantomData<T>),
}
impl<T: ErrorMetadata> ParseError<T> {
    pub fn try_from_error() -> Self {
        Self::TryFromError(PhantomData)
    }

    pub fn no_key_error() -> Self {
        Self::NoKeyError(PhantomData)
    }

    pub fn none_value_error() -> Self {
        Self::NoneValueError(PhantomData)
    }

    pub fn not_a_string_error() -> Self {
        Self::NotAStringError(PhantomData)
    }

    pub fn not_a_float_error() -> Self {
        Self::NotAFloatError(PhantomData)
    }
}

impl<T: ErrorMetadata> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromError(_) => {
                let wrapper = T::try_failure_wrapper();
                write!(f, "{}: Value is not an Object.", wrapper)
            },
            Self::NoKeyError(_) => {
                let wrapper = T::no_key_wrapper();
                let key = T::on_no_key();
                write!(f, "{}: Object no key \"{}\"", wrapper, key)
            },
            Self::NoneValueError(_) => {
                let wrapper = T::array_none_wrapper();
                write!(f, "{}: Value at index provided is None", wrapper)
            },
            Self::NotAStringError(_) => {
                let wrapper = T::not_a_string_wrapper();
                write!(f, "{}: Value is not a String", wrapper)
            },
            Self::NotAFloatError(_) => {
                let wrapper = T::not_a_float_wrapper();
                write!(f, "{}: String at array index is not a Number", wrapper)
            },
        }
    }
}

pub trait ErrorMetadata {
    // Wrapper called when try_from failed to receive a JSON Object.
    fn try_failure_wrapper() -> &'static str;
    // Wrapper called when the key does not exist.
    fn no_key_wrapper() -> &'static str;
    // Called to get the key name when the key hasn't been found.
    fn on_no_key() -> &'static str;
    // Wrapper called when the array doesn't have the given index.
    fn array_none_wrapper() -> &'static str;
    // Wrapper called when the elem in the array isn't a String.
    fn not_a_string_wrapper() -> &'static str;
    // Wrapper called when string is not a big decimal
    fn not_a_float_wrapper() -> &'static str;
}

pub struct AskInfoMetadata {}

impl AskInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing AskInfo"
    }
}

pub struct BidInfoMetadata {}
impl BidInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing BidInfo"
    }
}

pub struct HighInfoMetadata {}

impl HighInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing HighInfo"
    }
}

impl ErrorMetadata for AskInfoMetadata {
    // Wrapper called when try_from failed to receive a JSON Object.
    fn try_failure_wrapper() -> &'static str {
        Self::wrapper()
    }
    // Wrapper called when the key does not exist.
    fn no_key_wrapper() -> &'static str {
        Self::wrapper()
    }
    // Called to get the key name when the key hasn't been found.
    fn on_no_key() -> &'static str {
        "a"
    }
    // Wrapper called when the array doesn't have the given index.
    fn array_none_wrapper() -> &'static str {
        Self::wrapper()
    }
    // Wrapper called when the elem in the array isn't a String.
    fn not_a_string_wrapper() -> &'static str {
        Self::wrapper()
    }
    // Wrapper called when string is not a big decimal
    fn not_a_float_wrapper() -> &'static str {
        Self::wrapper()
    }
}

impl ErrorMetadata for BidInfoMetadata {
    // Wrapper called when try_from failed to receive a JSON Object.
    fn try_failure_wrapper() -> &'static str {
        Self::wrapper()
    }
    // Wrapper called when the key does not exist.
    fn no_key_wrapper() -> &'static str {
        Self::wrapper()
    }
    // Called to get the key name when the key hasn't been found.
    fn on_no_key() -> &'static str {
        "b"
    }
    // Wrapper called when the array doesn't have the given index.
    fn array_none_wrapper() ->  &'static str {
        Self::wrapper()
    }
    // Wrapper called when the elem in the array isn't a String.
    fn not_a_string_wrapper() ->  &'static str {
        Self::wrapper()
    }
    // Wrapper called when string is not a big decimal
    fn not_a_float_wrapper() ->  &'static str {
        Self::wrapper()
    }
}

impl ErrorMetadata for HighInfoMetadata {
    // Wrapper called when try_from failed to receive a JSON Object.
    fn try_failure_wrapper() ->  &'static str {
        Self::wrapper()
    }
    // Wrapper called when the key does not exist.
    fn no_key_wrapper() ->  &'static str {
        Self::wrapper()
    }
    // Called to get the key name when the key hasn't been found.
    fn on_no_key() ->  &'static str {
        "h"
    }
    // Wrapper called when the array doesn't have the given index.
    fn array_none_wrapper() ->  &'static str {
        Self::wrapper()
    }
    // Wrapper called when the elem in the array isn't a String.
    fn not_a_string_wrapper() ->  &'static str {
        Self::wrapper()
    }
    // Wrapper called when string is not a big decimal
    fn not_a_float_wrapper() ->  &'static str {
        Self::wrapper()
    }
}
