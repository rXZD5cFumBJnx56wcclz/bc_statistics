use crate::{orders::orders, pnl::pnl, prelude::*, utils::all};

pub fn pnl_on_orders(stat_collector: &StatCollector) -> Vec<f64> {
    all(&[pnl(stat_collector), orders(stat_collector)])
}
