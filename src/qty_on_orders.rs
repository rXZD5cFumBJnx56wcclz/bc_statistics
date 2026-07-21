use crate::{orders::orders, prelude::*, qty::qty, utils::all};

pub fn qty_on_orders<'a>(stat_collector: &StatCollector) -> Vec<f64> {
    all(&[qty(stat_collector), orders(stat_collector)])
}
