use crate::{instrument::FTXInstrument, types::ExchangeName};
use chrono::{DateTime, Utc};
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub enum PushMessage {
    OrderbookUpdate {
        exchange: ExchangeName,
        instrument: FTXInstrument,
        bid1: (Decimal, Decimal),
        ask1: (Decimal, Decimal),
        remote_updated_at: DateTime<Utc>,
        local_updated_at: DateTime<Utc>,
        local_sent_out_at: DateTime<Utc>,
    },
}
