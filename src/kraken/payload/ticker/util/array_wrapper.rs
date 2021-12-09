use super::parse_error::ParseError;
use super::unpack::try_from_map;
use super::ErrorWrapper;
use bigdecimal::BigDecimal;
use serde_json::Value;
use std::convert::TryFrom;
use std::marker::PhantomData;

pub struct ArrayWrapper<T, P, const N: usize>(Box<[T; N]>, PhantomData<P>);

impl<T, P, const N: usize> ArrayWrapper<T, P, N> {
    pub fn new(array: Box<[T; N]>) -> Self {
        Self(array, PhantomData)
    }
}

impl<T, P, const N: usize> From<ArrayWrapper<T, P, N>> for Box<[T; N]> {
    fn from(array: ArrayWrapper<T, P, N>) -> Self {
        array.0
    }
}

/*
impl<T, P, const N: usize> Into<[T; N]> for ArrayWrapper<T, P, N> {
    fn into(self) -> Box<[T; N]> {
        self.0.take()
    }
}
*/

impl<T: ErrorWrapper, const N: usize> TryFrom<&Value> for ArrayWrapper<BigDecimal, T, N> {
    type Error = ParseError<T>;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        // First, remove the map element from its Value wrapper.
        match val.as_object() {
            Some(obj) => try_from_map(obj).map(|val| ArrayWrapper::new(val)),
            None => Err(ParseError::<T>::try_from_error()),
        }
    }
}
