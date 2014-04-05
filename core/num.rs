// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use option::{Option, Some};

macro_rules! one_impl(
    ($tyname:ty) => (
        impl One for $tyname {
            #[inline(always)]
            fn one() -> $tyname {
                1 as $tyname
            }
        }
    )
)

macro_rules! to_prim_impl(
    ($tyname:ty) => (
        impl ToPrimitive for $tyname {
            #[inline(always)]
            fn to_uint(&self) -> Option<uint> {
                Some(*self as uint)
            }

            #[inline(always)]
            fn to_int(&self) -> Option<int> {
                Some(*self as int)
            }
        }
    )
)

macro_rules! int_module(
    ($tyname:ty) => (
        one_impl!($tyname)
        to_prim_impl!($tyname)
    )
)

pub trait One {
    fn one() -> Self;
}

pub trait ToPrimitive {
    fn to_uint(&self) -> Option<uint>;
    fn to_int(&self) -> Option<int>;
}

int_module!(u8)
int_module!(u16)
int_module!(u32)
int_module!(u64)
int_module!(uint)
int_module!(i8)
int_module!(i16)
int_module!(i32)
int_module!(i64)
int_module!(int)
int_module!(f32)
int_module!(f64)

