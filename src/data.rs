use bc_utils::other::transpose;

use crate::capital::Capital;
use crate::not_reduce_orders::NotReduceOrders;
use crate::orders::Orders;
use crate::pnl_on_orders::PnlOnOrders;
use crate::positions_index_sep::PositionsIndexSep;
use crate::prelude::*;
use crate::qty_on_orders::QtyOnOrders;
use crate::reduce_orders::ReduceOrders;
use crate::stat_data::StatData;

pub trait Data<'a>: Capital<'a> + PositionsIndexSep + Orders + PnlOnOrders + QtyOnOrders {
    fn data(&'a self) -> StatData;
}

impl<'a> Data<'a> for StatCollector<'a> {
    fn data(&'a self) -> StatData {
        StatData(vec![
            MAP_LINK::from_iter([
                (
                    "time",
                    (0..self.states.len())
                        .map(|v| v as f64)
                        .collect::<Vec<f64>>(),
                ),
                ("open", self.into_iter().map(|v| v.src[1]).collect()),
                ("high", self.into_iter().map(|v| v.src[2]).collect()),
                ("low", self.into_iter().map(|v| v.src[3]).collect()),
                ("close", self.into_iter().map(|v| v.src[4]).collect()),
                ("volume", self.into_iter().map(|v| v.src[5]).collect()),
                ("turnover", self.into_iter().map(|v| v.src[6]).collect()),
                ("capital", self.capital()),
                ("reduce_orders", self.reduce_orders()),
                ("not_reduce_orders", self.not_reduce_orders()),
                ("pnl_orders", self.pnl_on_orders()),
                ("qty_on_orders", self.qty_on_orders()),
            ]),
            {
                let mut bind = transpose(
                    self.positions_index_sep()
                        .into_iter()
                        .map(|(time, pos)| vec![time as f64, pos])
                        .collect::<Vec<Vec<f64>>>(),
                );
                if !bind.is_empty() {
                    MAP_LINK::from_iter([
                        ("time", bind.remove(0)),
                        ("positions_index_sep", bind.remove(0)),
                    ])
                } else {
                    MAP_LINK::from_iter([
                        ("time", Default::default()),
                        ("positions_index_sep", Default::default()),
                    ])
                }
            },
        ])
    }
}
