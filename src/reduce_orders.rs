use crate::prelude::*;

pub fn reduce_orders<'a>(stat_collector: &StatCollector) -> Vec<f64> {
    stat_collector
        .states
        .iter()
        .map(|s| {
            if s.orders
                .borrow()
                .values()
                .any(|o| !o.is_active && o.is_reduce)
            {
                s.src[1]
            } else {
                f64::NAN
            }
        })
        .collect()
}
