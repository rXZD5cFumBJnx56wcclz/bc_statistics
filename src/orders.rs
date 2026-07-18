use crate::{
    not_reduce_orders::NotReduceOrders, prelude::*, reduce_orders::ReduceOrders, utils::any,
};

pub trait Orders: NotReduceOrders + ReduceOrders {
    fn orders(&self) -> Vec<f64>;
}

impl Orders for StatCollector<'_> {
    fn orders(&self) -> Vec<f64> {
        any(&[self.reduce_orders(), self.not_reduce_orders()])
    }
}
