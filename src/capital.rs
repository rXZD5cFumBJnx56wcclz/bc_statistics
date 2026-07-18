use crate::prelude::*;

pub trait Capital<'a> {
    fn capital(&'a self) -> Vec<f64>;
}

impl<'a> Capital<'a> for StatCollector<'a> {
    fn capital(&'a self) -> Vec<f64> {
        self.into_iter().map(|v| v.capital).collect()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::prelude_tests::prelude::*;

//     #[test]
//     fn capital_res_1() {
//         assert_eq_pr!();
//     }
// }
