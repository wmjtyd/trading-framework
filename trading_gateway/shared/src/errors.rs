use crate::response::Response;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Debug, Error)]
pub enum MGError {
    #[error("Stop order unimplemented")]
    UnimplementedStopOrder,

    #[error("Not logged in")]
    NotLoggedIn,

    #[error("No position available")]
    NoPositionAvailable,

    #[error("Not implemented")]
    NotImplemented,

    #[error("Unsupported exchange")]
    UnsupportedExchange,

    #[error("Already logged in")]
    AlreadyLoggedIn,

    #[error("Invalid label '{0}'")]
    InvalidLabel(String),

    #[error("No such order '{0}'")]
    NoSuchOrder(String),

    #[error("No such order '{0}'")]
    NoSuchOrderByLocalId(u32),

    #[error("Local stop order exists: {0}")]
    StopOrderExist(String),

    #[error("Local open order exists: {0}")]
    OpenOrderExist(String),

    #[error("Local trade exists: {0}")]
    TradeExist(String),

    #[error("Order '{0}' is not managed by system")]
    NotManagedOrder(String),

    #[error("Cannot parse symbol '{0}'")]
    ParsePairError(String),

    #[error("Nothing to subscribe")]
    EmptyTopic,

    #[error("Cannot parse topic channel")]
    ParseTopiced,

    #[error("Cannot send message to channel")]
    SendError,

    #[error("Remote error")]
    Remote(String),

    #[error(transparent)]
    Timeout(#[from] Elapsed),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Prometheus(#[from] prometheus::Error),

    #[error(transparent)]
    FtxRest(#[from] ftx::rest::Error),

    #[error(transparent)]
    FtxWs(#[from] ftx::ws::Error),

    #[error(transparent)]
    Decimal(#[from] rust_decimal::Error),

    #[error(transparent)]
    Zmq(#[from] zmq::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

pub trait ToResponse {
    fn to_response(self) -> Response;
}

impl ToResponse for Result<Response, MGError> {
    fn to_response(self) -> Response {
        match self {
            Ok(resp) => resp,
            Err(e) => e.to_response(),
        }
    }
}

impl ToResponse for Result<(), MGError> {
    fn to_response(self) -> Response {
        match self {
            Ok(_) => Response::Success,
            Err(e) => e.to_response(),
        }
    }
}

impl ToResponse for MGError {
    fn to_response(self) -> Response {
        Response::Error(format!("{}", self))
    }
}
