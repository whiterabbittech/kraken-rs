use super::parse_error::ParseError;
use super::ErrorWrapper;
use bigdecimal::BigDecimal;
use serde_json::{Map, Value};
use std::str::FromStr;

pub fn unpack_decimal_array<T: ErrorWrapper, const N: usize>(
    array: &Value,
) -> Result<Box<[BigDecimal; N]>, ParseError<T>> {
    let unpacker = |i| unpack_decimal(array.get(i));
    (0..N)
        .into_iter()
        .map(unpacker)
        .collect::<Result<Vec<BigDecimal>, ParseError<T>>>()
        .map(vec_to_array)
}

pub fn unpack_u64_array<T: ErrorWrapper, const N: usize>(
    array: &Value,
) -> Result<Box<[u64; N]>, ParseError<T>> {
    let unpacker = |i| unpack_u64(array.get(i));
    (0..N)
        .into_iter()
        .map(unpacker)
        .collect::<Result<Vec<u64>, ParseError<T>>>()
        .map(vec_to_array)
}

pub fn unpack_u64<T: ErrorWrapper>(val: Option<&Value>) -> Result<u64, ParseError<T>> {
    match val {
        Some(v) => v.as_u64().ok_or_else(ParseError::<T>::not_a_u64_error),
        None => Err(ParseError::<T>::none_value_error()),
    }
}

pub fn unpack_decimal<T: ErrorWrapper>(val: Option<&Value>) -> Result<BigDecimal, ParseError<T>> {
    match val {
        Some(v) => unpack_unwrapped_decimal(v),
        None => Err(ParseError::<T>::none_value_error()),
    }
}

fn unpack_unwrapped_decimal<T: ErrorWrapper>(val: &Value) -> Result<BigDecimal, ParseError<T>> {
    match val {
        Value::String(decimal_str) => unpack_decimal_str(decimal_str),
        _ => Err(ParseError::<T>::not_a_string_error()),
    }
}

fn unpack_decimal_str<T: ErrorWrapper>(val: &str) -> Result<BigDecimal, ParseError<T>> {
    let parsed_decimal = BigDecimal::from_str(val);
    parsed_decimal.map_err(|_| ParseError::<T>::not_a_float_error())
}

pub fn try_from_map<T: ErrorWrapper, const N: usize>(
    obj: &Map<String, Value>,
) -> Result<Box<[BigDecimal; N]>, ParseError<T>> {
    let key = T::key();
    match obj.get(key) {
        Some(array) => unpack_decimal_array(array),
        None => Err(ParseError::<T>::no_key_error()),
    }
}

pub fn try_from_map_u64<T: ErrorWrapper, const N: usize>(
    obj: &Map<String, Value>,
) -> Result<Box<[u64; N]>, ParseError<T>> {
    let key = T::key();
    match obj.get(key) {
        Some(array) => unpack_u64_array(array),
        None => Err(ParseError::<T>::no_key_error()),
    }
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> Box<[T; N]> {
    // else-case is what happens when the vector isn't the right length.
    let else_case = |v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len());
    // Jam this vector into an array.
    let array = v.try_into().unwrap_or_else(else_case);
    Box::new(array)
}
