use bc_trade_state::utils::pnl as pnl_util;

use crate::prelude::*;

pub fn pnl(stat_collector: &StatCollector) -> Vec<f64> {
    stat_collector
        .states
        .iter()
        .zip(stat_collector.src.iter())
        .map(|(s, src)| {
            if let Some(p) = s.positions.borrow().values().next() {
                // Indicators will not be completely accurate if the statistics are used in real trading.
                pnl_util(p.qty, p.avg_open_price, src[1], p.leverage, &p.side).1
            } else {
                f64::NAN
            }
        })
        .collect()
}
