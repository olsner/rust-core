// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use c_types::c_int;

mod detail {
    extern {
        pub fn abort(msg: &str) -> !;
        pub fn breakpoint();
    }
}

#[inline(never)]
pub fn abort() -> ! {
    abort2("aborted. no message.");
}

pub fn abort2(msg: &str) -> ! {
    unsafe { detail::abort(msg); }
}

pub fn breakpoint() {
    unsafe { detail::breakpoint() }
}

#[inline]
#[lang="fail_bounds_check"]
pub fn fail_bounds_check(_file: *u8, _line: uint, _index: uint, _len: uint) -> ! {
    abort2("fail_bounds_check");
}

#[inline]
#[lang="fail_"]
pub fn fail_(_expr: *u8, _file: *u8, _line: uint) -> ! {
    abort2("fail_");
}

#[inline]
pub fn out_of_memory() -> ! {
    abort2("out of memory")
}

#[cfg(debug)]
#[inline(always)]
pub fn assert(b: bool) {
    if !b {
        abort("assert failed")
    }
}

#[cfg(not(debug))]
#[inline(always)]
pub fn assert(_: bool) {
}

pub static EINTR: c_int = 4;
pub static EBUSY: c_int = 16;
pub static ETIMEDOUT: c_int = 110;
