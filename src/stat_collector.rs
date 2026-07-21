use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct StatCollector<'a> {
    pub symbol: String,
    pub states: Vec<TradeState<'a>>,
    pub ind: Vec<MAP<&'a str, f64>>,
    pub signals: Vec<MAP<&'a str, Signal>>,
    pub signals_train: Vec<MAP<&'a str, f64>>,
}

impl<'a> StatCollector<'a> {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            states: Vec::new(),
            ind: Default::default(),
            signals: Default::default(),
            signals_train: Default::default(),
        }
    }
    pub fn push(
        &mut self,
        state: TradeState<'a>,
        ind: MAP<&'a str, f64>,
        signals: MAP<&'a str, Signal>,
        signals_train: MAP<&'a str, f64>,
    ) {
        self.states.push(state);
        self.ind.push(ind);
        self.signals.push(signals);
        self.signals_train.push(signals_train);
    }
}

impl<'a, 'b> IntoIterator for &'a StatCollector<'b> {
    type Item = &'a TradeState<'b>;
    type IntoIter = std::slice::Iter<'a, TradeState<'b>>;

    fn into_iter(self) -> Self::IntoIter {
        self.states.iter()
    }
}
