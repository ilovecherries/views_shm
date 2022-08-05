use std::borrow::{BorrowMut};
use std::ffi::{c_void, CStr, OsStr};
use std::os::raw::{c_char, c_int, c_uint};
use std::os::windows::ffi::OsStrExt;
use std::ptr::addr_of;
use windows::core::{PCWSTR};
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::Memory::{CreateFileMappingW, FILE_MAP_ALL_ACCESS, MapViewOfFile, PAGE_READWRITE};
use widestring::{encode_utf16, utf16str};

#[no_mangle]
pub extern "C" fn hello_world() {
	println!("Hello!");
}

/// This is just a placeholder until we come up with a better value
const BUF_SIZE: usize = 256;

/// Shared memory region in which the programs communicate through.
///
/// This will be updated by `SharedMemoryContainer` and it will also handle any writes and reads.
#[repr(C)]
#[derive(Copy, Clone)]
struct SharedMemory {
	/// The lock for the shared memory region to prevent race conditions
	lock: bool,
	/// The internal data
	data: [u8; BUF_SIZE],
}

impl Default for SharedMemory {
	fn default() -> Self {
		SharedMemory {
			lock: false,
			data: [0; 256],
		}
	}
}

/// This contains an internal image of the shared memory that isn't meant to be interacted with.
#[derive(Copy, Clone)]
struct SharedMemoryContainer {
	/// The key for creating a handle to the shared memory.
	///
	/// Mostly stored for debugging purposes.
	key: PCWSTR,
	/// Is the process the host for the shared memory? (created with `create_shared_memory`).
	is_host: bool,
	/// The handle to the shared memory.
	file_mapping: HANDLE,
}

impl SharedMemoryContainer {
	/// Overwrite the memory held in the shared memory buffer
	fn write(self) {
		// TODO: we will need to do something with mutexes or locks in the future for critical data
		let p = unsafe {
			MapViewOfFile(
				self.file_mapping,
				FILE_MAP_ALL_ACCESS,
				0,
				0,
				BUF_SIZE
			)
		};
	}
}

/// This converts a string to a vector of wide characters (UTF-16)
fn to_wchar(str: &str) -> Vec<u16> {
	OsStr::new(str)
		.encode_wide()
		.chain(Some(0).into_iter())
		.collect()
}

/// This converts a string to a PCWSTR
macro_rules! pcwstr {
	($str:expr) => {
		PCWSTR(to_wchar(unsafe { CStr::from_ptr($str) }.to_str().unwrap()).as_ptr())
	};
}

static mut MEMORY: Option<SharedMemoryContainer> = None;

/// This creates a shared memory region using the given key in Windows and sets it up so that
/// the program running this library is the host.
///
/// Apparently processes also need this privilege?: <https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants>
///
/// Ref: <https://docs.microsoft.com/en-us/windows/win32/memory/creating-named-shared-memory>
///
/// If there was a failure creating the file mapping, it will return -1.
#[no_mangle]
pub extern "C" fn create_shared_memory(key: *const c_char, size: c_uint) -> c_int {
	// let name = utf16str!("Global\\MyFileMappingObject");
	let name = pcwstr!(key);
	let attributes: SECURITY_ATTRIBUTES = Default::default();
	match unsafe {
		CreateFileMappingW(
			INVALID_HANDLE_VALUE,
			addr_of!(attributes),
			PAGE_READWRITE,
			0,
			size as u32,
			name,
		)
	} {
		Ok(h) => {
			let memory: SharedMemory = Default::default();
			
			unsafe {
				// SAFETY!: Shouldn't name be freed once it reaches this block???
				// I really need to test this... My IDE not giving any warnings (neither is compiler
				// so I guess I'm fine) (YES I TRIED THIS OUTSIDE AN UNSAFE BLOCK>>>>)
				MEMORY = Some(SharedMemoryContainer {
					key: name,
					is_host: true,
					file_mapping: h,
				});
			}
			0
		}
		Err(e) => {
			println!("{e}");
			-1
		}
	}
}

/// This writes to the shared memory region.
///
/// If you are not the host / did not create a shared memory region, this will -1.
#[no_mangle]
pub unsafe extern "C" fn write_shared_memory(data: *mut c_void) -> c_int {
	match MEMORY {
		Some(mut m) => {
			// m.borrow_mut().lock();
			// m.borrow_mut().unlock();
			
			0
		}
		None => -1
	}
}