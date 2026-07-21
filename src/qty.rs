use crate::{prelude::*, utils::value_from_pos};

pub fn qty(stat_collector: &StatCollector) -> Vec<f64> {
    value_from_pos(stat_collector, |p| p.qty)
}
