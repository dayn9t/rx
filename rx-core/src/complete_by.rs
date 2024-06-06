pub use rx_core_derive::CompleteBy;

/// 通过另一个实例补全当前实例
pub trait CompleteBy {
    fn complete_by(&mut self, other: &Self);
}
macro_rules! impl_complete_by {
    ($($t:ty),* $(,)?) => {
        $(
            impl CompleteBy for $t {
                fn complete_by(&mut self, other: &Self) {
                    if *self == Self::default() {
                        *self = *other;
                    }
                }
            }
        )*
    };
}

impl_complete_by!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

impl<T: Clone> CompleteBy for Option<T> {
    fn complete_by(&mut self, other: &Self) {
        if self.is_none() {
            *self = other.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::complete_by::CompleteBy;

    #[test]
    fn int_complete() {
        let mut a = 10;
        let b = 0;
        a.complete_by(&b);
        assert_eq!(a, 10);

        let a = 10u8;
        let mut b = 0u8;
        b.complete_by(&a);
        assert_eq!(b, 10);
    }
    #[test]
    fn option_complete() {
        let mut a = Some(10);
        let mut b = None;
        a.complete_by(&b);
        assert_eq!(a, Some(10));

        a = Some(10);
        b = None;
        b.complete_by(&a);
        assert_eq!(b, Some(10));

        a = None;
        b = None;
        b.complete_by(&a);
        assert_eq!(b, None);
    }


    #[derive(Debug, CompleteBy)]
    pub struct Opt {
        pub domain: Option<String>,
        pub root: Option<PathBuf>,
        pub verbose: u8,
    }

}
