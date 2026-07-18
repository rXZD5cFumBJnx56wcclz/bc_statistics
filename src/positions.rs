use crate::{prelude::*, some::Some_};

pub trait Positions: Some_ {
    fn positions(&self) -> Vec<f64>;
}

impl Positions for StatCollector<'_> {
    fn positions(&self) -> Vec<f64> {
        self.some(|t| t.positions.borrow(), false)
    }
}
