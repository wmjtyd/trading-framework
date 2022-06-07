use crate::errors::MGError;
use crate::FTXInstrument;
use chrono::{DateTime, Utc};
use fehler::throws;
use ftx::rest::{OrderStatus as FtxOrderStatus, OrderType as FtxOrderType};
use ftx::ws::{OrderInfo, Side};
use rkyv::{Archive, Deserialize, Serialize};
use rust_decimal::Decimal;
use std::convert::{TryFrom, TryInto};

#[derive(Archive, Deserialize, Serialize, Debug, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
    StopMarket,
    StopLimit,
}

#[derive(Archive, Deserialize, Serialize, Debug, Copy, Clone)]
pub enum OrderStatus {
    Submitted,    // locally create the order entry
    Acknowledged, // the order is acknowledged by the exchange
    PartialFilled,
    Closed,
}

#[derive(Archive, Deserialize, Serialize, Debug, Copy, Clone)]
pub enum LiquidityType {
    Taker,
    Maker,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct OrderUpdate {
    pub id: String,
    pub client_id: Option<String>,
    pub instrument: FTXInstrument,
    pub r#type: OrderType,
    pub status: OrderStatus,
    pub price: Option<Decimal>,
    pub avg_fill_price: Option<Decimal>,
    pub qty: Decimal,
    pub filled_qty: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct AddTrade {
    id: String,
    order_id: String,
    instrument: FTXInstrument,
    price: Option<Decimal>,
    qty: Decimal,
    liquidity: LiquidityType,
    fee_rate: Decimal,
    time: DateTime<Utc>,
}

// Market order from FTX: type: Market, status: Closed
// Cancel order from FTX: status: Closed, filled_qty: 0, avg_fill_price: None,
// Place order from FTX: status: New, filled_qty: 0, avg_fill_price: None,
// update order from FTX: close the old one and then open a new one.
impl TryFrom<OrderInfo> for OrderUpdate {
    type Error = MGError;

    #[throws(MGError)]
    fn try_from(oi: OrderInfo) -> Self {
        OrderUpdate {
            id: oi.id.to_string(),
            instrument: oi.market.parse()?,
            r#type: oi.r#type.try_into()?,
            status: oi.status.try_into()?,
            price: oi.price,
            avg_fill_price: oi.avg_fill_price,
            qty: match oi.side {
                Side::Buy => oi.size,
                Side::Sell => -oi.size,
            },
            filled_qty: match oi.side {
                Side::Buy => oi.filled_size.unwrap(),
                Side::Sell => -oi.filled_size.unwrap(),
            },
            client_id: oi.client_id,
            created_at: oi.created_at,
        }
    }
}

impl TryFrom<FtxOrderType> for OrderType {
    type Error = MGError;

    #[throws(MGError)]
    fn try_from(ty: FtxOrderType) -> Self {
        match ty {
            FtxOrderType::Limit => OrderType::Limit,
            FtxOrderType::Market => OrderType::Market,
            _ => todo!(),
        }
    }
}

impl TryFrom<FtxOrderStatus> for OrderStatus {
    type Error = MGError;

    #[throws(MGError)]
    fn try_from(ty: FtxOrderStatus) -> Self {
        match ty {
            FtxOrderStatus::New => OrderStatus::Acknowledged,
            FtxOrderStatus::Open => OrderStatus::PartialFilled,
            FtxOrderStatus::Closed => OrderStatus::Closed,
        }
    }
}
