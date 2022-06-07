use derive_more::Display;
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::ops::{Neg, Not};

pub type OrderId = String;
pub type LocalId = u32;
pub type TradeId = String;

#[derive(Copy, Clone, Debug, Display, Archive, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ExchangeName {
    FTX,
    Binance,
}

#[derive(Debug, Clone, Display, Copy, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

impl Not for Side {
    type Output = Side;
    fn not(self) -> Side {
        match self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}

impl Neg for Side {
    type Output = Side;
    fn neg(self) -> Side {
        match self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}

pub type AccountName = String;

pub trait Sign {
    fn sign(self) -> Self;
}

impl Sign for Decimal {
    fn sign(self) -> Decimal {
        match self.cmp(&Decimal::ZERO) {
            Ordering::Greater => Decimal::ONE,
            Ordering::Equal => Decimal::ZERO,
            Ordering::Less => Decimal::NEGATIVE_ONE,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::Sign;
    use rust_decimal::Decimal;

    #[test]
    fn test_decimal_sign() {
        assert_eq!(Decimal::NEGATIVE_ONE.sign(), Decimal::NEGATIVE_ONE);
        assert_eq!(Decimal::ONE_HUNDRED.sign(), Decimal::ONE);
        assert_eq!(Decimal::ZERO.sign(), Decimal::ZERO);
    }
}
