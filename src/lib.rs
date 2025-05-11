#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;
use core::mem::take;
pub trait Shufl {
    type Elem;
    fn push(&mut self, e: Self::Elem) -> Result<(), impl Iterator<Item = Self::Elem>>;
    fn drain(&mut self) -> impl Iterator<Item = Self::Elem>;
}
impl<T, const N: usize> Shufl for heapless::Vec<T, N> {
    type Elem = T;

    fn push(&mut self, e: Self::Elem) -> Result<(), impl Iterator<Item = Self::Elem>> {
        match self.push(e) {
            Ok(a) => Ok(a),
            Err(val) => Err(take(self).into_iter().chain([val])),
        }
    }

    fn drain(&mut self) -> impl Iterator<Item = Self::Elem> {
        take(self).into_iter()
    }
}
#[cfg(feature = "alloc")]
const _: () = {
    use core::iter::Empty;

    use alloc::vec::Vec;

    impl<T> Shufl for Vec<T> {
        type Elem = T;

        fn push(&mut self, e: Self::Elem) -> Result<(), impl Iterator<Item = Self::Elem>> {
            self.push(e);
            Ok::<_, Empty<T>>(())
        }

        fn drain(&mut self) -> impl Iterator<Item = Self::Elem> {
            self.drain(..)
        }
    }
};
