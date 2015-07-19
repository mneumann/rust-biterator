#![feature(zero_one)]
use std::iter::Iterator;
use std::num::{Zero, One};
use std::ops::{BitAnd, BitOr, Shr, Shl};

pub struct BiteratorLsb<T> {
    val: T,
    at_end: bool
}

impl<T> BiteratorLsb<T>
    where T : Sized+Eq+Shr<usize, Output=T>+Ord+BitAnd<T, Output=T>+Zero+One+Copy
{
    pub fn new(init: T) -> BiteratorLsb<T> {
        BiteratorLsb{val: init, at_end: false}
    }
}

impl<T> Iterator for BiteratorLsb<T>
    where T : Sized+Eq+Shr<usize, Output=T>+Ord+BitAnd<T, Output=T>+Zero+One+Copy
{
    type Item = bool; 

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end { return None; }

        let lsb = self.val & T::one();
        self.val = self.val >> 1usize; 
        if self.val == T::zero() {
            self.at_end = true;
        }

        if lsb == T::zero() {
            Some(false)
        } else {
            Some(true)
        }
    }
}

pub fn build_from<T, I>(iter: &mut I, max: usize) -> T 
    where I: Iterator<Item=bool>,
          T: Zero + One + BitOr<T, Output=T> + Shl<usize, Output=T>
{
    let mut val: T = T::zero();

    let mut i = 0;
    loop {
        if i >= max { break; }
        let bit = match iter.next() {
            Some(true)  => T::one(),
            Some(false) => T::zero(),
            None => break
        };
        val = val | (bit << i);
        i += 1;
    }

    return val;
}

#[test]
fn test_bitstream() {
    let v: Vec<_> = BiteratorLsb::new(12u64).collect();
    assert_eq!([false, false, true, true], &v[..]);

    let v: Vec<_> = BiteratorLsb::new(0b1110u8).collect();
    assert_eq!([false, true, true, true], &v[..]);

    let v: Vec<_> = BiteratorLsb::new(0b001110u8).collect();
    assert_eq!([false, true, true, true], &v[..]);
}

#[test]
fn test_build_from() {
    let num = 0b001110u8;
    let n = build_from(&mut BiteratorLsb::new(num), 8);
    assert_eq!(num, n);
}

#[test]
fn test_build_from_partial() {
    let mut iter = BiteratorLsb::new(0b1110u8);
    assert_eq!(0b0u8, build_from(&mut iter, 1));
    assert_eq!(0b11u8, build_from(&mut iter, 2));
    assert_eq!(0b1u8, build_from(&mut iter, 1));
}
