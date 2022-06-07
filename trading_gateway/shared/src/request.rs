use crate::{instrument::FTXInstrument, types::ExchangeName};
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Archive, Deserialize, Serialize, Debug)]
pub enum Request {
    Login {
        exchange: ExchangeName,
        account: String,
        key: String,
        secret: String,
    },
    Logout {
        exchange: ExchangeName,
        account: String,
    },
    IsLoggedIn {
        exchange: ExchangeName,
        account: String,
    },
    PlaceOrder {
        exchange: ExchangeName,
        account: String,
        instrument: FTXInstrument,
        price: Option<Decimal>,
        qty: Decimal,
        post_only: bool,
    },
    CancelOrder {
        exchange: ExchangeName,
        account: String,
        order_id: String,
    },
    CancelAllOrders {
        exchange: ExchangeName,
        account: String,
        instrument: Option<FTXInstrument>,
    },
    GetOpenOrders {
        exchange: ExchangeName,
        account: String,
        instrument: Option<FTXInstrument>,
    },
    GetPosition {
        exchange: ExchangeName,
        account: String,
        instrument: FTXInstrument,
        virt: bool,
    },
    HasOpenOrder {
        exchange: ExchangeName,
        account: String,
        instrument: FTXInstrument,
    },
}

impl Request {
    pub fn exchange(&self) -> Option<ExchangeName> {
        match self {
            Request::Login { exchange, .. } => Some(*exchange),
            Request::Logout { exchange, .. } => Some(*exchange),
            Request::IsLoggedIn { exchange, .. } => Some(*exchange),
            Request::PlaceOrder { exchange, .. } => Some(*exchange),
            Request::CancelOrder { exchange, .. } => Some(*exchange),
            Request::CancelAllOrders { exchange, .. } => Some(*exchange),
            Request::GetPosition { exchange, .. } => Some(*exchange),
            Request::GetOpenOrders { exchange, .. } => Some(*exchange),
            Request::HasOpenOrder { exchange, .. } => Some(*exchange),
        }
    }
}
