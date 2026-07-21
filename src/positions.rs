use crate::{prelude::*, utils::some};

pub fn positions(stat_collector: &StatCollector) -> Vec<f64> {
    some(stat_collector, |t| t.positions.borrow(), false)
}
