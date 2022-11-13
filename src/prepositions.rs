use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Prepositions {
    pub(crate) almost: Vec<String>,
    pub(crate) exactly: Vec<String>,
    pub(crate) roughly: Vec<String>,
    pub(crate) prepo_err: Vec<String>,
}
