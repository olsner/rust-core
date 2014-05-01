// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mem::{forget, uninit};
use cmp::{Eq, Ord};

extern "rust-intrinsic" {
    pub fn offset<T>(dst: *T, offset: int) -> *T;
    pub fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *T, count: uint);
    pub fn copy_memory<T>(dst: *mut T, src: *T, count: uint);
    pub fn set_memory<T>(dst: *mut T, val: u8, count: uint);
}

#[inline]
pub unsafe fn read_ptr<T>(src: *T) -> T {
    let mut tmp: T = uninit();
    copy_nonoverlapping_memory(&mut tmp, src, 1);
    tmp
}

#[inline]
pub unsafe fn swap_ptr<T>(x: *mut T, y: *mut T) {
    let mut tmp: T = uninit();

    copy_nonoverlapping_memory(&mut tmp, x as *T, 1);
    copy_memory(x, y as *T, 1); // `x` and `y` may overlap
    copy_nonoverlapping_memory(y, &tmp, 1);

    forget(tmp);
}

impl<T> Eq for *T {
    #[inline(always)]
    fn eq(&self, other: &*T) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &*T) -> bool { *self != *other }
}

impl<T> Eq for *mut T {
    #[inline(always)]
    fn eq(&self, other: &*mut T) -> bool { *self == *other }

    #[inline(always)]
    fn ne(&self, other: &*mut T) -> bool { *self != *other }
}

impl<T> Ord for *T {
    #[inline(always)]
    fn lt(&self, other: &*T) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &*T) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &*T) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &*T) -> bool { *self >= *other }
}

impl<T> Ord for *mut T {
    #[inline(always)]
    fn lt(&self, other: &*mut T) -> bool { *self < *other }

    #[inline(always)]
    fn le(&self, other: &*mut T) -> bool { *self <= *other }

    #[inline(always)]
    fn gt(&self, other: &*mut T) -> bool { *self > *other }

    #[inline(always)]
    fn ge(&self, other: &*mut T) -> bool { *self >= *other }
}

pub fn null<T>() -> *T { 0 as *T }
pub fn mut_null<T>() -> *mut T { 0 as *mut T }

pub trait RawPtr<T> {
	fn null() -> Self;
	fn is_null(&self) -> bool;
	fn is_not_null(&self) -> bool { !self.is_null() }

	unsafe fn offset(self, count : int) -> Self;
}

impl<T> RawPtr<T> for *mut T {
	fn null() -> *mut T { mut_null() }
	fn is_null(&self) -> bool { *self == mut_null() }

	unsafe fn offset(self, count : int) -> *mut T {
		offset(self as *T, count) as *mut T
	}
}

impl<T> RawPtr<T> for *T {
	fn null() -> *T { null() }
	fn is_null(&self) -> bool { *self == null() }

	unsafe fn offset(self, count : int) -> *T {
		offset(self as *T, count) as *T
	}
}
