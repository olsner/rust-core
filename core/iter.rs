// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clone::Clone;
use cmp::Ord;
use num::One;
use option::{Option, Some, None};
use ops::Add;

pub trait Iterator<A> {
    fn next(&mut self) -> Option<A>;

    /// Return a lower bound and upper bound on the remaining length of the iterator.
    ///
    /// The common use case for the estimate is pre-allocating space to store the results.
    #[inline(always)]
    fn size_hint(&self) -> (uint, Option<uint>) { (0, None) }

    #[inline]
    fn fold<B>(&mut self, init: B, f: |B, A| -> B) -> B {
        let mut accum = init;
        loop {
            match self.next() {
                Some(x) => { accum = f(accum, x); }
                None    => { break; }
            }
        }
        accum
    }

    #[inline]
    fn all(&mut self, f: |A| -> bool) -> bool {
        for x in *self { if !f(x) { return false; } }
        true
    }

    #[inline]
    fn any(&mut self, f: |A| -> bool) -> bool {
        for x in *self { if f(x) { return true; } }
        false
    }
}

pub trait DoubleEndedIterator<A>: Iterator<A> {
    fn next_back(&mut self) -> Option<A>;

    #[inline(always)]
    fn invert(self) -> Invert<Self> {
        Invert { iter: self }
    }
}

#[deriving(Clone)]
pub struct Range<T> {
    priv low: T,
    priv high: T,
    priv step: T
}

#[deriving(Clone)]
pub struct Invert<T> {
    priv iter: T
}

impl<A: Add<A, A> + Ord + Clone> Iterator<A> for Range<A> {
    fn next(&mut self) -> Option<A> {
        if self.low < self.high {
            let val = self.low.clone();
            self.low = self.low + self.step;
            Some(val)
        } else {
            None
        }
    }
}

impl<A, T: DoubleEndedIterator<A>> Iterator<A> for Invert<T> {
    #[inline(always)]
    fn next(&mut self) -> Option<A> { self.iter.next_back() }

    #[inline(always)]
    fn size_hint(&self) -> (uint, Option<uint>) { self.iter.size_hint() }
}

impl<A, T: DoubleEndedIterator<A>> DoubleEndedIterator<A> for Invert<T> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<A> { self.iter.next() }
}

#[inline(always)]
pub fn range<A: Add<A, A> + Ord + Clone + One>(start: A, stop: A) -> Range<A> {
    range_step(start, stop, One::one())
}

#[inline(always)]
pub fn range_inclusive<A: Add<A, A> + Ord + Clone + One>(start: A, stop: A) -> Range<A> {
    range_step_inclusive(start, stop, One::one())
}

#[inline(always)]
pub fn range_step<A: Add<A, A> + Ord + Clone>(start: A, stop: A, step: A) -> Range<A> {
    Range {
        low: start,
        high: stop,
        step: step
    }
}

#[inline(always)]
pub fn range_step_inclusive<A: Add<A, A> + Ord + Clone + One>(start: A, stop: A, step: A) -> Range<A> {
    Range {
        low: start,
        high: stop + One::one(),
        step: step
    }
}

