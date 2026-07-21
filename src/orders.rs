use crate::{
    not_reduce_orders::not_reduce_orders, prelude::*, reduce_orders::reduce_orders, utils::any,
};

pub fn orders(stat_collector: &StatCollector) -> Vec<f64> {
    any(&[
        reduce_orders(stat_collector),
        not_reduce_orders(stat_collector),
    ])
}
