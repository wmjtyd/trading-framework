use rand::Rng;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::hash::{Hash, Hasher};

pub trait ClientLabel:
    Hash + Eq + Sized + Serialize + DeserializeOwned + Clone + std::fmt::Debug
// + Archive
// + RkyvDeserialize
// + RkyvSerialize
{
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct Label<L> {
    #[serde(rename = "i")]
    id: u32,
    #[serde(rename = "c")]
    pub client_label: Option<L>,
}

impl<L> Hash for Label<L>
where
    L: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.client_label.hash(state)
    }
}

impl<L> PartialEq for Label<L>
where
    L: PartialEq,
{
    fn eq(&self, other: &Label<L>) -> bool {
        self.client_label == other.client_label
    }
}

impl<L> Label<L> {
    pub fn new(client_label: Option<L>) -> Label<L> {
        Label {
            id: Self::gen_id(),
            client_label,
        }
    }

    pub fn gen_id() -> u32 {
        rand::thread_rng().gen::<u32>()
    }
}

impl ClientLabel for String {}

impl ClientLabel for () {}
