use num_traits::Float;

pub trait NumsExt<T> {
    fn del_nan(self, sep: usize) -> impl Iterator<Item = (usize, T)>;
}

impl<T: Float + Default, V: Iterator<Item = T>> NumsExt<T> for V {
    fn del_nan(self, sep: usize) -> impl Iterator<Item = (usize, T)> {
        self.enumerate()
            .scan(0usize, move |num, el| {
                if el.1.is_normal() {
                    Some((el.0, el.1))
                } else {
                    *num += 1;
                    if *num <= sep {
                        Some((el.0, el.1))
                    } else {
                        *num = 0;
                        Some((Default::default(), T::nan()))
                    }
                }
            })
            .filter(|v| v.1.is_normal())
    }
}
