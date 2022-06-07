use super::{Instrument, InstrumentType, PositionUpdatePolicy};
use crate::errors::MGError;
use fehler::throws;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq)]
pub struct BinanceInstrument {
    base: String,
    quote: String,
}

impl FromStr for BinanceInstrument {
    type Err = MGError;

    #[allow(unreachable_code)]
    #[throws(MGError)]
    fn from_str(s: &str) -> BinanceInstrument {
        static RE: OnceCell<Regex> = OnceCell::new();

        let re = RE
            .get_or_try_init(|| Regex::new(r"^([A-Z]{3})([A-Z]{3,4})$"))
            .unwrap();

        if let Some(cap) = re.captures(s) {
            let mut iter = cap.iter();
            let _ = iter.next();

            let base = iter.next().unwrap().unwrap();
            let quote = iter.next().unwrap().unwrap();
            return BinanceInstrument {
                base: base.as_str().to_string(),
                quote: quote.as_str().to_string(),
            };
        }
        unimplemented!()
    }
}

impl Instrument for BinanceInstrument {
    fn quote(&self) -> String {
        self.quote.clone()
    }

    fn base(&self) -> String {
        self.base.clone()
    }

    fn r#type(&self) -> InstrumentType {
        InstrumentType::Spot
    }

    fn position_update_policy(&self) -> PositionUpdatePolicy {
        unimplemented!()
    }
}

impl fmt::Display for BinanceInstrument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}
