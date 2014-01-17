// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! one_impl(
    ($tyname:ty) => (
        impl One for $tyname {
            fn one() -> $tyname {
                1 as $tyname
            }
        }
    )
)

pub trait One {
    fn one() -> Self;
}

one_impl!(u8)
one_impl!(u16)
one_impl!(u32)
one_impl!(u64)
one_impl!(uint)
one_impl!(i8)
one_impl!(i16)
one_impl!(i32)
one_impl!(i64)
one_impl!(int)
one_impl!(f32)
one_impl!(f64)

