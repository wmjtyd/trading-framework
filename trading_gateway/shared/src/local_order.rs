use super::instrument::FTXInstrument;
use super::label::Label;
use super::local_trade::LocalTrade;
use super::types::Sign;
use super::{OrderId, TradeId};
use chrono::{DateTime, Utc};
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;
use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct LocalOrder<L> {
    pub order_id: OrderId,
    pub instrument: FTXInstrument,
    pub price: Option<Decimal>,
    pub avg_fill_price: Option<Decimal>,
    pub qty: Decimal,
    pub filled_qty: Decimal,
    pub label: Option<Label<L>>,
    pub trades: HashMap<TradeId, LocalTrade>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<L> LocalOrder<L> {
    pub fn new(
        instrument: FTXInstrument,
        order_id: &str,
        price: Option<Decimal>,
        avg_fill_price: Option<Decimal>,
        qty: Decimal,
        filled_qty: Decimal,
        label: Option<Label<L>>,
    ) -> Self {
        Self {
            order_id: order_id.to_string(),
            instrument,
            price,
            avg_fill_price,
            qty,
            filled_qty,
            label,
            trades: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn remain_qty(&self) -> Decimal {
        self.qty - self.filled_qty
    }

    pub fn remain_qty_from_trade(&self) -> Decimal {
        let LocalOrder {
            ref trades,
            qty: mut remain_qty,
            ..
        } = self;
        let sign = remain_qty.sign();
        for trade in trades.values() {
            remain_qty -= trade.qty;
            assert!(sign * remain_qty >= Decimal::ZERO);
        }

        remain_qty
    }
}

impl<L> PartialOrd for LocalOrder<L> {
    fn partial_cmp(&self, other: &LocalOrder<L>) -> Option<Ordering> {
        self.order_id.partial_cmp(&other.order_id)
    }
}

impl<L> Ord for LocalOrder<L> {
    fn cmp(&self, other: &LocalOrder<L>) -> Ordering {
        self.order_id.cmp(&other.order_id)
    }
}

impl<L> PartialEq for LocalOrder<L> {
    fn eq(&self, other: &LocalOrder<L>) -> bool {
        self.order_id == other.order_id
    }
}

impl<L> Eq for LocalOrder<L> {}
