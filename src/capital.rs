use crate::prelude::*;

pub fn capital(stat_collector: &StatCollector) -> Vec<f64> {
    stat_collector.states.iter().map(|v| v.capital).collect()
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
