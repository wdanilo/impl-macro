pub use impl_macro_internal::*;

pub trait Abs {
    fn abs(self) -> Self;
}

imp! { Abs
    for bool,
    for char,
    for u8,
    for u16,
    for u32,
    for u64,
    for u128,
    for usize where bool: std::fmt::Display, usize: std::fmt::Display, {
        fn abs(self) -> Self {
            self
        }
    }
}
