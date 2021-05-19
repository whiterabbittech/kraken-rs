use std::fmt;

pub enum AssetPair {
    EthUsd,
    BtcUsd,
    AlgoUsd,
    DotUsd,
}

impl fmt::Display for AssetPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Self::EthUsd => "ETHUSD",
            Self::BtcUsd => "BTCUSD",
            Self::DotUsd => "DOTUSD",
            Self::AlgoUsd => "ALGOUSD",
        };
        write!(f, "{}", val)
    }
}
