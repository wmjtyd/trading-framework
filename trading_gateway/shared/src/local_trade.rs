use super::TradeId;
use chrono::{DateTime, Utc};
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;
use std::cmp::{Eq, Ord, Ordering, PartialOrd};

#[derive(Clone, Archive, Deserialize, Serialize)]
pub struct LocalTrade {
    pub id: TradeId,
    pub price: Decimal,
    pub qty: Decimal,
    pub time: DateTime<Utc>,
}

impl PartialOrd for LocalTrade {
    fn partial_cmp(&self, other: &LocalTrade) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Ord for LocalTrade {
    fn cmp(&self, other: &LocalTrade) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialEq for LocalTrade {
    fn eq(&self, other: &LocalTrade) -> bool {
        self.id == other.id
    }
}
impl Eq for LocalTrade {}

impl LocalTrade {
    pub fn new(id: &str, price: Decimal, qty: Decimal, time: DateTime<Utc>) -> Self {
        Self {
            id: id.into(),
            price,
            qty,
            time,
        }
    }
}

impl std::fmt::Display for LocalTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}: {:.2} @ {:.1}>", self.id, self.price, self.qty)
    }
}

impl std::fmt::Debug for LocalTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
