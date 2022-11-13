use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct SpecialCases {
    pub(crate) before_midnight: String,
    pub(crate) after_midnight: String,
    pub(crate) midnight: String,
    pub(crate) two_to_one: String,
    pub(crate) one_to_one: String,
    pub(crate) exactly_one: String,
    pub(crate) one_past_one: String,
    pub(crate) two_past_one: String,
    pub(crate) noon: String,
}
