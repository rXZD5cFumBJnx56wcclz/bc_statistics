pub fn all(values: &[Vec<f64>]) -> Vec<f64> {
    let first = values.first().unwrap();
    (0..first.len())
        .map(|i| {
            if values.iter().all(|v| v[i].is_normal()) {
                first[i]
            } else {
                f64::NAN
            }
        })
        .collect()
}
pub fn any(values: &[Vec<f64>]) -> Vec<f64> {
    (0..values.first().unwrap().len())
        .map(|i| {
            let bind = values.iter().map(|v| v[i]).find(|v| v.is_normal());
            if let Some(el) = bind {
                el
            } else {
                f64::NAN
            }
        })
        .collect()
}
