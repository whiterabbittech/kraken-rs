use std::fmt;
use std::marker::PhantomData;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use serde_json::Value;
use std::convert::TryFrom;

pub type AskError = ParseError<AskInfoMetadata>;
pub type BidError = ParseError<BidInfoMetadata>;
pub type HighError = ParseError<HighInfoMetadata>;
pub type LowError = ParseError<LowInfoMetadata>;
pub type LastTradeError = ParseError<LastTradeInfoMetadata>;

pub enum ParseError<T: ErrorMetadata> {
    TryFrom(PhantomData<T>),
    NoKey(PhantomData<T>),
    NoneValue(PhantomData<T>),
    NotAString(PhantomData<T>),
    NotAFloat(PhantomData<T>),
}

impl<T: ErrorMetadata> ParseError<T> {
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

impl<T: ErrorMetadata> fmt::Display for ParseError<T> {
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

impl<T: ErrorMetadata> fmt::Debug for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
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

pub trait ErrorWrapper {    
    fn wrapper() -> &'static str;    
    fn key() -> &'static str;
}

impl <T: ErrorWrapper> ErrorMetadata for T {
    fn try_failure_wrapper() -> &'static str {
        Self::wrapper()
    }
    fn no_key_wrapper() -> &'static str {
        Self::wrapper()
    }
    fn on_no_key() -> &'static str {
        Self::key()
    }
    fn array_none_wrapper() -> &'static str {
        Self::wrapper()
    }
    fn not_a_string_wrapper() -> &'static str {
        Self::wrapper()
    }
    fn not_a_float_wrapper() -> &'static str {
        Self::wrapper()
    }
}

pub struct AskInfoMetadata {}

impl ErrorWrapper for AskInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing AskInfo"
    }

    fn key() -> &'static str {
        "a"
    }
}

pub struct BidInfoMetadata {}
impl ErrorWrapper for BidInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing BidInfo"
    }

    fn key() -> &'static str {
        "b"
    }
}

pub struct HighInfoMetadata {}

impl ErrorWrapper for HighInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing HighInfo"
    }
    fn key() -> &'static str {
        "h"
    }
}

pub struct LowInfoMetadata {}

impl ErrorWrapper for LowInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing LowInfo"
    }
    fn key() -> &'static str {
        "l"
    }
}

pub struct LastTradeInfoMetadata {}

impl ErrorWrapper for LastTradeInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing LastTradeInfo"
    }

    fn key() -> &'static str {
        "c"
    }
}

pub fn unpack_decimal<T: ErrorMetadata>(val: Option<&Value>) -> Result<BigDecimal, ParseError<T>> {
    match val {
        Some(v) => unpack_unwrapped_decimal(v),
        None => Err(ParseError::<T>::none_value_error()),
    }
}

fn unpack_unwrapped_decimal<T: ErrorMetadata>(val: &Value) -> Result<BigDecimal, ParseError<T>> {
    match val {
        Value::String(decimal_str) => unpack_decimal_str(decimal_str),
        _ => Err(ParseError::<T>::not_a_string_error()),
    }
}

fn unpack_decimal_str<T: ErrorMetadata>(val: &str) -> Result<BigDecimal, ParseError<T>> {
    let parsed_decimal = BigDecimal::from_str(val);
    parsed_decimal.map_err(|_| ParseError::<T>::not_a_float_error())
}

pub fn unpack_decimal_array<T: ErrorMetadata, const N: usize>(array: &Value) -> Result<[BigDecimal; N], ParseError<T>> {
// pub fn unpack_decimal_array<T: ErrorMetadata>(array: &Value) -> Result<Vec<BigDecimal>, ParseError<T>> {
    let results: Result<Vec<BigDecimal>, ParseError<T>> = (0..N).into_iter().map(|i| {
        let decimal_str = array.get(i); 
        unpack_decimal(decimal_str)
    }).collect();
    let array: Result<[BigDecimal; N], ParseError<T>> = results.map(|vector| vector.try_into().unwrap());
    array

    // Hmm... I wonder if there's a fancy functional way to do
    // this in the 2021 Edition.
    //0..N.iter().map(|i| unpack_decimal(array.get(i))) 
    // let mut decimals = [BigDecimal::default(); N];
    // Let's get N values out of the array.
    // Parse each of them as BigDecimals.
    //for i in 0..N {
        //let decimal_str = array.get(i);
        //decimals[i] = unpack_decimal(decimal_str)?;
    //}
    //Ok(decimals)
    
}
