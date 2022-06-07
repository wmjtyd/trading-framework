use super::instrument::FTXInstrument;
use super::label::Label;
use super::local_order::LocalOrder;
use super::OrderId;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct LocalStopOrder<L> {
    pub order_id: OrderId,
    pub ins: FTXInstrument,
    pub price: Option<Decimal>,
    pub stop: Decimal,
    pub qty: Decimal,
    pub label: Option<Label<L>>,
    pub created_at: DateTime<Utc>,
}

impl<L> LocalStopOrder<L> {
    pub fn new(
        ins: FTXInstrument,
        order_id: &str,
        price: Option<Decimal>,
        stop: Decimal,
        qty: Decimal,
        label: Option<Label<L>>,
    ) -> Self {
        Self {
            order_id: order_id.to_string(),
            ins,
            price,
            stop,
            qty,
            label,
            created_at: Utc::now(),
        }
    }

    pub fn trigger(self) -> LocalOrder<L> {
        LocalOrder::new(
            self.ins,
            &self.order_id,
            self.price,
            None,
            self.qty,
            self.qty,
            self.label,
        )
    }
}
