mod errors;
mod instrument;
mod ipc;
mod label;
mod local_order;
mod local_stop_order;
mod local_trade;
mod order;
mod push;
mod request;
mod response;
mod types;

pub use errors::*;
pub use instrument::*;
pub use ipc::*;
pub use label::*;
pub use local_order::LocalOrder;
pub use local_stop_order::LocalStopOrder;
pub use local_trade::LocalTrade;
pub use order::*;
pub use push::*;
pub use request::*;
pub use response::*;
pub use types::*;

pub type Result<T> = std::result::Result<T, MGError>;
