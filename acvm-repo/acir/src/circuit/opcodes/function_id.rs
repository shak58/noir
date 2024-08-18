use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash, Default, PartialOrd, Ord)]
#[serde(transparent)]
pub struct AcirFunctionId(pub u32);

impl AcirFunctionId {
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for AcirFunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AcirFunctionId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>().map(AcirFunctionId)
    }
}
