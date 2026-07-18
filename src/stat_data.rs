use std::ops::Deref;

use crate::prelude::*;

#[derive(PartialEq, Debug, Default)]
pub struct StatData(pub Vec<MAP_LINK<&'static str, Vec<f64>>>);

impl StatData {
    pub fn to_vec(&self) -> Vec<Vec<Vec<f64>>> {
        self.0
            .iter()
            .map(|v| v.iter().map(|v| v.1.clone()).collect::<Vec<Vec<f64>>>())
            .collect::<Vec<Vec<Vec<f64>>>>()
    }
}

impl Deref for StatData {
    type Target = Vec<MAP_LINK<&'static str, Vec<f64>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
