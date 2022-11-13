use serde::{Deserialize, Serialize};

use crate::{
    hours::Hours, minutes::Minutes, prepositions::Prepositions, special_cases::SpecialCases,
};

// TODO implement structs properly
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub(crate) struct Template {
    pub language: String,
    pub hours: Hours,
    pub minutes: Minutes,
    pub prepositions: Prepositions,
    pub special_cases: SpecialCases,
    pub start_sentence: Vec<String>,
    pub end_sentence: Vec<String>,
    pub on_the_hour_template: Vec<String>,
    pub normal_template: Vec<String>,
}
