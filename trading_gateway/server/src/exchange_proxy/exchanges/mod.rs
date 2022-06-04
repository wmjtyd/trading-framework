use crate::prelude::internal::*;
use async_trait::async_trait;
use rust_decimal::Decimal;
use anyhow::Result;

pub type ExchangeName = String;
pub type Instrument = String;
pub type OrderId = String;

#[async_trait]
pub trait ExchangeProxy {
    fn name(&self) -> ExchangeName;
    fn accounts(&self) -> Vec<String>;

    async fn connect(&mut self) -> Result<()>;
    async fn login(&mut self, account_name: &str, key: &str, secret: &str) -> Result<()>;
    async fn logout(&mut self, account_name: &str) -> Result<()>;
    fn is_logged_in(&self, account_name: &str) -> Result<bool>;
    async fn tick(&mut self) -> Result<()>;
    async fn new_order(
        &mut self,
        account: &str,
        instrument: &Instrument,
        price: Option<Decimal>,
        qty: Decimal,
        post_only: bool,
    ) -> Result<OrderId>;

    async fn cancel_order(&mut self, account: &str, order_id: &str) -> Result<()>;
    async fn cancel_all_orders(
        &mut self,
        account: &str,
        market: Option<Instrument>,
    ) -> Result<()>;

    async fn stop(&mut self) -> Result<()>;
    async fn account_value(&self, account_name: &str) -> Result<Decimal>;

    fn clear_logins(&mut self) -> Result<()>;
}
