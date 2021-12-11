pub trait ErrorWrapper {
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
        Self::key()
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
    /// wrapper is the error message that wraps the underlying error.
    fn wrapper() -> &'static str;
    /// key is the name of the key into the object which we're parsing.
    fn key() -> &'static str;
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

pub struct NumTradesInfoMetadata {}

impl ErrorWrapper for NumTradesInfoMetadata {
    fn wrapper() -> &'static str {
        "Error Parsing NumTrades"
    }
    fn key() -> &'static str {
        "t"
    }
}
