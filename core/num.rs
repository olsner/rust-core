// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! define_int_one(
    ($tyname:ty) => (
        impl One for $tyname {
            fn one() -> $tyname {
                1
            }
        }
    )
)

macro_rules! define_float_one(
    ($tyname:ty) => (
        impl One for $tyname {
            fn one() -> $tyname {
                1.0
            }
        }
    )
)

pub trait One {
    fn one() -> Self;
}

define_int_one!(u8)
define_int_one!(u16)
define_int_one!(u32)
define_int_one!(u64)
define_int_one!(uint)
define_int_one!(i8)
define_int_one!(i16)
define_int_one!(i32)
define_int_one!(i64)
define_int_one!(int)
define_float_one!(f32)
define_float_one!(f64)

