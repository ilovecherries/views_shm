use std::borrow::{BorrowMut};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::ptr::addr_of;
use windows::core::{PCWSTR};
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Memory::{CreateFileMappingW, PAGE_READWRITE};
use widestring::utf16str;

#[no_mangle]
pub extern "C" fn hello_world() {
	println!("Hello!");
}

/// Shared memory region in which the programs communicate through.
#[repr(C)]
#[derive(Copy, Clone)]
struct SharedMemory {
	/// The lock for the shared memory region to prevent race conditions
	lock: bool,
}

impl SharedMemory {
	fn lock(&mut self) {
		self.lock = true;
	}
	
	fn unlock(&mut self) {
		self.lock = false;
	}
}

static mut HANDLE2: Option<HANDLE> = None;
static mut SHARED: Option<SharedMemory> = None;

/// This creates a shared memory region using the given key in Windows and sets it up so that
/// the program running this library is the host.
///
/// If there was a failure creating the file mapping, it will return -1.
pub extern "C" fn create_shared_memory(key: *mut c_char) -> c_int {
	let name = utf16str!("Global\\MyFileMappingObject");
	let attributes: SECURITY_ATTRIBUTES = Default::default();
	match unsafe {
		CreateFileMappingW(
			INVALID_HANDLE_VALUE,
			addr_of!(attributes),
			PAGE_READWRITE,
			0,
			256,
			PCWSTR(name.as_ptr()),
		)
	} {
		Ok(h) => {
			unsafe { HANDLE2 = h; }
			0
		}
		Err(_) => -1
	}
}

/// This writes to the shared memory region.
///
/// If you are not the host / did not create a shared memory region, this will -1.
pub unsafe extern "C" fn write_shared_memory(data: *mut c_void) -> c_int {
	match SHARED {
		Some(mut m) => {
			m.borrow_mut().lock();
			0
		}
		None => -1
	}
}