extern crate core;

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};
use libloading::{Library, Symbol};

type HelloFunc = unsafe fn();
type CreateShmFunc = unsafe fn(*const c_char, c_uint) -> c_int;

fn main() {
	unsafe {
		let lib = Library::new("shmlib.dll").unwrap();
		let create_shared_memory: Symbol<CreateShmFunc> = lib.get(b"create_shared_memory").unwrap();
		const size: usize = 256;
		match create_shared_memory(
			unsafe { CString::new("Global\\MyFileMappingObject").unwrap() }.as_ptr(),
			size as c_uint,
		) {
			0 => println!("Successful!@"),
			_ => println!("Not successful!"),
		};
	}
}
