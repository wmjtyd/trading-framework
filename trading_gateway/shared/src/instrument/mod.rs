mod binance;
mod ftx;

use crate::types::Sign;
use derive_more::Display;
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;
use std::hash::Hash;

pub use self::binance::BinanceInstrument;
pub use self::ftx::FTXInstrument;

#[derive(
    Archive, Deserialize, Serialize, Debug, Clone, Copy, Display, Hash, PartialEq, Eq, PartialOrd,
)]
pub enum PositionUpdatePolicy {
    Linear,
    Inverse,
}

pub const NULL_POSITION: (Decimal, Decimal) = (Decimal::ZERO, Decimal::ZERO);

pub trait Instrument: Hash + Eq + Clone + std::fmt::Display + Send {
    fn base(&self) -> String;
    fn quote(&self) -> String;
    fn r#type(&self) -> InstrumentType;
    fn position_update_policy(&self) -> PositionUpdatePolicy;

    fn update_position(
        &self,
        current: (Decimal, Decimal),
        incoming: (Decimal, Decimal),
    ) -> (Decimal, Decimal) {
        let policy = self.position_update_policy();

        match policy {
            PositionUpdatePolicy::Linear => {
                if incoming.1 == Decimal::ZERO {
                    current
                } else if current.1 == Decimal::ZERO {
                    if incoming.1 == Decimal::ZERO {
                        NULL_POSITION
                    } else {
                        (incoming.0, incoming.1) // New position
                    }
                } else if current.1.sign() != incoming.1.sign() {
                    let new_qty = current.1 + incoming.1; // Close position
                    if new_qty == Decimal::ZERO {
                        NULL_POSITION // Fully closed
                    } else {
                        (current.0, new_qty)
                    }
                } else if current.1.sign() == incoming.1.sign() {
                    let new_avg = (current.0 * current.1 + incoming.0 * incoming.1)
                        / (current.1 + incoming.1);
                    (new_avg, current.1 + incoming.1) // Open position
                } else {
                    unreachable!()
                }
            }
            PositionUpdatePolicy::Inverse => {
                if incoming.1 == Decimal::ZERO {
                    current
                } else if current.1 == Decimal::ZERO {
                    if incoming.1 == Decimal::ZERO {
                        NULL_POSITION
                    } else {
                        (incoming.0, incoming.1) // New position
                    }
                } else if current.1.sign() != incoming.1.sign() {
                    let new_qty = current.1 + incoming.1; // Close position
                    if new_qty == Decimal::ZERO {
                        NULL_POSITION // Fully closed
                    } else {
                        (current.0, new_qty)
                    }
                } else if current.1.sign() == incoming.1.sign() {
                    let new_avg = (current.1 + incoming.1)
                        / (current.1 / current.0 + incoming.1 / incoming.0);
                    (new_avg, current.1 + incoming.1) // Open position
                } else {
                    unreachable!()
                }
            }
        }
    }
}

#[derive(
    Archive, Deserialize, Serialize, Debug, Clone, Copy, Display, Hash, PartialEq, Eq, PartialOrd,
)]
pub enum InstrumentType {
    Spot,
    Future,
}
