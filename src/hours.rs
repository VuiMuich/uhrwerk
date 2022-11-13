use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Hours {
    pub(crate) one: String,
    pub(crate) two: String,
    pub(crate) three: String,
    pub(crate) four: String,
    pub(crate) five: String,
    pub(crate) six: String,
    pub(crate) seven: String,
    pub(crate) eight: String,
    pub(crate) nine: String,
    pub(crate) ten: String,
    pub(crate) eleven: String,
    pub(crate) twelve: String,
    pub(crate) hour_err: String,
}
