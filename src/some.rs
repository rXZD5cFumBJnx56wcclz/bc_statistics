use crate::prelude::*;

pub trait Some_ {
    fn some<'a, K, T>(
        &'a self,
        func: fn(&'a TradeState) -> Ref<'a, MAP<K, T>>,
        only_active: bool,
    ) -> Vec<f64>
    where
        T: IsActive;
}

impl Some_ for StatCollector<'_> {
    fn some<'a, K, T>(
        &'a self,
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
        self.states
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
}
