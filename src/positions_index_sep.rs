use crate::{impls::NumsExt, positions::Positions, prelude::*};

pub trait PositionsIndexSep: Positions {
    fn positions_index_sep(&self) -> Vec<(usize, f64)>;
}

impl PositionsIndexSep for StatCollector<'_> {
    fn positions_index_sep(&self) -> Vec<(usize, f64)> {
        self.positions().into_iter().del_nan(1).collect()
    }
}
