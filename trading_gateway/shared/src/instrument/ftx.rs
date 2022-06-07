use super::{Instrument, InstrumentType, PositionUpdatePolicy};
use crate::errors::MGError;
use fehler::throws;
use once_cell::sync::OnceCell;
use regex::Regex;
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Archive, Deserialize, Serialize, Debug, Clone, Hash, PartialEq, PartialOrd, Eq)]
pub struct FTXInstrument {
    base: String,
    quote: String,
    r#type: InstrumentType,
    expiry: Option<String>,
}

impl FromStr for FTXInstrument {
    type Err = MGError;

    #[allow(unreachable_code)]
    #[throws(MGError)]
    fn from_str(s: &str) -> FTXInstrument {
        static RE_SPOT: OnceCell<Regex> = OnceCell::new();

        let re = RE_SPOT
            .get_or_try_init(|| Regex::new(r"^([A-Z]{1,4})/([A-Z]{1,4})$"))
            .unwrap();

        if let Some(cap) = re.captures(s) {
            let mut iter = cap.iter();
            let _ = iter.next();

            let base = iter.next().unwrap().unwrap();
            let quote = iter.next().unwrap().unwrap();
            return FTXInstrument {
                base: base.as_str().to_string(),
                quote: quote.as_str().to_string(),
                r#type: InstrumentType::Spot,
                expiry: None,
            };
        }

        static RE_FUTURE: OnceCell<Regex> = OnceCell::new();

        let re = RE_FUTURE
            .get_or_try_init(|| Regex::new(r"^([A-Z]{1,4})-([A-Z0-9]{4})$"))
            .unwrap();

        if let Some(cap) = re.captures(s) {
            let mut iter = cap.iter();
            let _ = iter.next();

            let sym = iter.next().unwrap().unwrap();
            let expiry = iter.next().unwrap().unwrap();

            let inst = match expiry.as_str() {
                "PERP" => FTXInstrument {
                    base: sym.as_str().to_string(),
                    quote: "USD".to_string(),
                    r#type: InstrumentType::Future,
                    expiry: Some("PERP".to_string()),
                },
                date => {
                    assert!(date.len() == 4);
                    FTXInstrument {
                        base: sym.as_str().to_string(),
                        quote: "USD".to_string(),
                        r#type: InstrumentType::Future,
                        expiry: Some(date.to_string()),
                    }
                }
            };

            return inst;
        }

        unimplemented!("Unsupported instrument {}", s)
    }
}

impl Instrument for FTXInstrument {
    fn quote(&self) -> String {
        self.quote.clone()
    }

    fn base(&self) -> String {
        self.base.clone()
    }

    fn r#type(&self) -> InstrumentType {
        self.r#type
    }

    fn position_update_policy(&self) -> PositionUpdatePolicy {
        PositionUpdatePolicy::Linear
    }
}

impl fmt::Display for FTXInstrument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.r#type {
            InstrumentType::Future => {
                if let Some(ref expiry) = self.expiry {
                    write!(f, "{}-{}", self.base, expiry)
                } else {
                    write!(f, "{}-PERP", self.base)
                }
            }
            InstrumentType::Spot => {
                write!(f, "{}/{}", self.base, self.quote)
            }
        }
    }
}
