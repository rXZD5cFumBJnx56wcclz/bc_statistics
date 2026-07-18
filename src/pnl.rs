use bc_trade_state::utils::pnl;

use crate::prelude::*;

pub trait Pnl {
    fn pnl(&self) -> Vec<f64>;
}

impl Pnl for StatCollector<'_> {
    fn pnl(&self) -> Vec<f64> {
        self.states
            .iter()
            .map(|s| {
                if let Some(p) = s.positions.borrow().values().next() {
                    // Indicators will not be completely accurate if the statistics are used in real trading.
                    pnl(p.qty, p.avg_open_price, s.src[1], p.leverage, &p.side).1
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
}
