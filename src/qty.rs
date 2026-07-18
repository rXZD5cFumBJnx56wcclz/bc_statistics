use crate::{prelude::*, value_from_pos::ValueFromPos};

pub trait Qty: ValueFromPos {
    fn qty(&self) -> Vec<f64>;
}

impl Qty for StatCollector<'_> {
    fn qty(&self) -> Vec<f64> {
        self.value_from_pos(|p| p.qty)
    }
}
