use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct StatCollector<'a> {
    pub symbol: String,
    pub states: Vec<TradeState<'a>>,
    pub ind: Vec<MAP<&'a str, f64>>,
    pub signals_ready: Vec<MAP<&'a str, Signal>>,
    pub signals_train: Vec<MAP<&'a str, f64>>,
}

impl<'a> StatCollector<'a> {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            states: Vec::new(),
            ind: Default::default(),
            signals_ready: Default::default(),
            signals_train: Default::default(),
        }
    }
    pub fn push(
        &mut self,
        state: TradeState<'a>,
        ind: MAP<&'a str, f64>,
        signals_ready: MAP<&'a str, Signal>,
        signals_train: MAP<&'a str, f64>,
    ) {
        self.states.push(state);
        self.ind.push(ind);
        self.signals_ready.push(signals_ready);
        self.signals_train.push(signals_train);
    }
}

impl<'a> IntoIterator for &'a StatCollector<'a> {
    type Item = &'a TradeState<'a>;
    type IntoIter = std::slice::Iter<'a, TradeState<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.states).into_iter()
    }
}
