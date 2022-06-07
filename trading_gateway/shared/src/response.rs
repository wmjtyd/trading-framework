use super::local_order::LocalOrder;
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Archive, Deserialize, Serialize, Debug)]
pub enum Response {
    PlaceOrder { id: String },
    GetPosition { price: Decimal, qty: Decimal },
    GetOpenOrders { orders: Vec<LocalOrder<String>> },
    Success,
    Bool(bool),
    Error(String),
}
