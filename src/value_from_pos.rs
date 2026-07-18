use crate::prelude::*;

pub trait ValueFromPos {
    fn value_from_pos<F>(
        &self,
        f: F,
    ) -> Vec<f64>
    where
        F: Fn(&Position) -> f64;
}

impl ValueFromPos for StatCollector<'_> {
    fn value_from_pos<F>(
        &self,
        f: F,
    ) -> Vec<f64>
    where
        F: Fn(&Position) -> f64,
    {
        self.states
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
}
