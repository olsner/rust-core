#[no_std];

extern "rust-intrinsic" {
    fn offset<T>(dst: *T, offset: int) -> *T;
}

type c_int = i32;

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *u8, n: int) {
    unsafe {
        let mut i = 0;
        while (i < n) {
            *(offset(dest as *u8, i) as *mut u8) = *(offset(src, i));
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn memmove(dest: *mut u8, src: *u8, n: int) {
    unsafe {
        if src < dest as *u8 { // copy from end
            let mut i = n;
            while (i != 0) {
                i -= 1;
                *(offset(dest as *u8, i) as *mut u8) = *(offset(src, i));
            }
        } else { // copy from beginning
            let mut i = 0;
            while (i < n) {
                *(offset(dest as *u8, i) as *mut u8) = *(offset(src, i));
                i += 1;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: c_int, n: int) {
    unsafe {
        let mut i = 0;
        while (i < n) {
            *(offset(s as *u8, i) as *mut u8) = c as u8;
            i += 1;
        }
    }
}
