use crate::{impls::NumsExt, positions::positions, prelude::*};

pub fn index_sep_positions(stat_collector: &StatCollector) -> Vec<f64> {
    positions(stat_collector)
        .into_iter()
        .del_nan(1)
        .map(|(i, v)| if v.is_normal() { i as f64 } else { f64::NAN })
        .collect()
}
