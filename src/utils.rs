use crate::prelude::*;

pub fn all(values: &[Vec<f64>]) -> Vec<f64> {
    let first = values.first().unwrap();
    (0..first.len())
        .map(|i| {
            if values.iter().all(|v| v[i].is_normal()) {
                first[i]
            } else {
                f64::NAN
            }
        })
        .collect()
}
pub fn any(values: &[Vec<f64>]) -> Vec<f64> {
    (0..values.first().unwrap().len())
        .map(|i| {
            let bind = values.iter().map(|v| v[i]).find(|v| v.is_normal());
            if let Some(el) = bind { el } else { f64::NAN }
        })
        .collect()
}

pub fn some<'a, K, T>(
    stat_collector: &'a StatCollector,
    func: fn(&'a TradeState) -> Ref<'a, MAP<K, T>>,
    only_active: bool,
) -> Vec<f64>
where
    T: IsActive,
{
    let f = |v: Ref<MAP<K, T>>| {
        if only_active {
            v.values().any(|v| v.is_active())
        } else {
            !v.is_empty()
        }
    };
    stat_collector
        .states
        .iter()
        .map(|c| {
            if f(func(c)) {
                // stat used open prices
                c.src[1]
            } else {
                f64::NAN
            }
        })
        .collect()
}

pub fn value_from_pos<F>(stat_collector: &StatCollector, f: F) -> Vec<f64>
where
    F: Fn(&Position) -> f64,
{
    stat_collector
        .states
        .iter()
        .map(|s| {
            if let Some(p) = s.positions.borrow().values().next() {
                f(p)
            } else {
                f64::NAN
            }
        })
        .collect()
}
