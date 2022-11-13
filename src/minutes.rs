use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Minutes {
    pub(crate) five_past: String,
    pub(crate) ten_past: String,
    pub(crate) quarter_past: String,
    pub(crate) twenty_past: String,
    pub(crate) twenty_five_past: String,
    pub(crate) half_past: String,
    pub(crate) twenty_five_to: String,
    pub(crate) twenty_to: String,
    pub(crate) quarter_to: String,
    pub(crate) ten_to: String,
    pub(crate) five_to: String,
    pub(crate) mini_err: String,
}
