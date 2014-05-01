// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ptr::copy_nonoverlapping_memory;
use cmp::min;

mod detail {
    extern "rust-intrinsic" {
        pub fn size_of<T>() -> uint;
        pub fn min_align_of<T>() -> uint;
        pub fn pref_align_of<T>() -> uint;
    }
}

#[inline(always)]
pub fn size_of<T>() -> uint {
    unsafe { detail::size_of::<T>() }
}

#[inline(always)]
pub fn size_of_val<T>(_: &T) -> uint {
    size_of::<T>()
}

pub fn nonzero_size_of<T>() -> uint {
    min(size_of::<T>(), 1)
}

#[inline(always)]
pub fn nonzero_size_of_val<T>(_: &T) -> uint {
    nonzero_size_of::<T>()
}

#[inline(always)]
pub fn min_align_of<T>() -> uint {
    unsafe { detail::min_align_of::<T>() }
}

#[inline(always)]
pub fn min_align_of_val<T>(_: &T) -> uint {
    min_align_of::<T>()
}

#[inline(always)]
pub fn pref_align_of<T>() -> uint {
    unsafe { detail::pref_align_of::<T>() }
}

#[inline(always)]
pub fn pref_align_of_val<T>(_: &T) -> uint {
    pref_align_of::<T>()
}

extern "rust-intrinsic" {
    pub fn forget<T>(_: T) -> ();
    pub fn transmute<T, U>(thing: T) -> U;
    pub fn init<T>() -> T;
    pub fn uninit<T>() -> T;
    pub fn move_val_init<T>(dst: &mut T, src: T);
    pub fn volatile_load<T>(src: *T) -> T;
    pub fn volatile_store<T>(dst: *mut T, val: T);
}

/// Coerce an immutable reference to be mutable.
#[inline(always)]
pub unsafe fn transmute_mut<'a,T>(ptr: &'a T) -> &'a mut T { transmute(ptr) }

pub fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let mut tmp: T = uninit();

        copy_nonoverlapping_memory(&mut tmp, x as *mut T as *T, 1);
        copy_nonoverlapping_memory(x, y as *mut T as *T, 1);
        copy_nonoverlapping_memory(y, &tmp, 1);

        forget(tmp);
    }
}

#[inline(always)]
pub fn replace<T>(dest: &mut T, mut src: T) -> T {
    swap(dest, &mut src);
    src
}

#[inline]
pub fn drop<T>(_ : T) {}
