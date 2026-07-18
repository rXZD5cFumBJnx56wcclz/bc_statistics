use crate::{orders::Orders, pnl::Pnl, prelude::*, utils::all};

pub trait PnlOnOrders: Pnl + Orders {
    fn pnl_on_orders(&self) -> Vec<f64>;
}

impl PnlOnOrders for StatCollector<'_> {
    fn pnl_on_orders(&self) -> Vec<f64> {
        all(&[self.pnl(), self.orders()])
    }
}
