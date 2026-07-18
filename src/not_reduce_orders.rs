use crate::prelude::*;

pub trait NotReduceOrders {
    fn not_reduce_orders(&self) -> Vec<f64>;
}

impl NotReduceOrders for StatCollector<'_> {
    fn not_reduce_orders(&self) -> Vec<f64> {
        self.states
            .iter()
            .map(|s| {
                if s.orders
                    .borrow()
                    .values()
                    .any(|o| !o.is_active && !o.is_reduce)
                {
                    s.src[1]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
}
