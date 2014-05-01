extern "rust-intrinsic" {
	pub fn set_memory<T>(dst: *mut T, val: u8, count: uint);
	pub fn copy_memory<T>(dst: *mut T, src: *T, count : uint);
}
