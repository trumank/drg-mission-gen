use serde::{Deserialize, Serialize};

use crate::cleaned_deep_dive::DeepDive;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DeepDivePair {
    pub(crate) normal: DeepDive,
    pub(crate) elite: DeepDive,
}
