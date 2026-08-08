use crate::prelude::*;

#[derive(Debug, PartialEq, Default)]
pub struct StatCollector<'a> {
    pub symbol: String,
    pub src: Vec<Vec<f64>>,
    pub ind: Vec<MAP<&'a str, f64>>,
    pub signals_train: Vec<MAP<&'a str, f64>>,
    pub signals: Vec<MAP<&'a str, Signal>>,
    pub utils_state: Vec<MAP<&'a str, f64>>,
    pub orders: Vec<MAP<&'a str, (Order, bool, Option<Trigger>)>>,
    pub states: Vec<TradeState<'a>>,
}

impl<'a> StatCollector<'a> {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }
    pub fn push(
        &mut self,
        src: Vec<f64>,
        ind: Option<MAP<&'a str, f64>>,
        signals_train: Option<MAP<&'a str, f64>>,
        signals: Option<MAP<&'a str, Signal>>,
        utils_state: Option<MAP<&'a str, f64>>,
        orders: Option<MAP<&'a str, (Order, bool, Option<Trigger>)>>,
        state: Option<TradeState<'a>>,
    ) {
        self.src.push(src);
        if let Some(ind) = ind {
            self.ind.push(ind);
        }
        if let Some(signals_train) = signals_train {
            self.signals_train.push(signals_train);
        }
        if let Some(signals) = signals {
            self.signals.push(signals);
        }
        if let Some(utils_state) = utils_state {
            self.utils_state.push(utils_state);
        }
        if let Some(orders) = orders {
            self.orders.push(orders);
        }
        if let Some(state) = state {
            self.states.push(state);
        }
    }
}
