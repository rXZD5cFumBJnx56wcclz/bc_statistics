use crate::prelude::*;

pub fn not_reduce_orders<'a>(stat_collector: &StatCollector) -> Vec<f64> {
    stat_collector
        .states
        .iter()
        .zip(stat_collector.src.iter())
        .map(|(s, src)| {
            if s.orders
                .borrow()
                .values()
                .any(|o| !o.is_active && !o.is_reduce)
            {
                src[1]
            } else {
                f64::NAN
            }
        })
        .collect()
}
