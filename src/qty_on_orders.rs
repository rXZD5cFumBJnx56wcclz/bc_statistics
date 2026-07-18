use crate::{orders::Orders, prelude::*, qty::Qty, utils::all};

pub trait QtyOnOrders: Qty {
    fn qty_on_orders(&self) -> Vec<f64>;
}

impl QtyOnOrders for StatCollector<'_> {
    fn qty_on_orders(&self) -> Vec<f64> {
        all(&[self.qty(), self.orders()])
    }
}
