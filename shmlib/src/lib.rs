// i really don't care about this anymore im just making it a set size get fucked

use std::os::raw::c_int;

use lua::{ffi::lua_State, State};
use shared_memory::{Shmem, ShmemConf};

const W: usize = 400;
const H: usize = 300;
/// Width of the game to render
const WIDTH: usize = 8 + W + 8;
/// Height of the game to render
const HEIGHT: usize = 8 + H + 8;
/// Width of the menu to render.
const MENU_WIDTH: usize = 400;
/// Height of the menu to render.
const MENU_HEIGHT: usize = 156;
/// Width of the window/viewport to create.
const WINDOW_WIDTH: usize = MENU_WIDTH;
/// Height of the window/viewport to create.
const WINDOW_HEIGHT: usize = 300 + MENU_HEIGHT;

#[repr(C)]
struct SharedMemoryBuffer {
	/// Graphic data
	///
	/// (this is hardcoded length so it can interact with the library)
	data: [u32; WINDOW_WIDTH * WINDOW_HEIGHT],
}

static mut SHM: Option<Shmem> = None;

pub extern "C" fn hello_world() {
	println!("OwO");
}

pub extern "C" fn establish_connection() -> c_int {
	-1
}

extern "C" fn read_shm(l: *mut lua_State) -> c_int {
	let mut state = unsafe { State::from_ptr(l) };
	let data = unsafe {
		std::ptr::read_volatile(SHM.as_ref().unwrap().as_ptr() as *mut SharedMemoryBuffer)
	}
	.data;
	state.new_table();
	for (i, d) in data.iter().enumerate() {
		state.push_number(*d as f64);
		state.raw_seti(-1, 1 + i as i64);
	}
	1
}

pub extern "C" fn luaopen_libshmlib(l: *mut lua_State) -> c_int {
	let mut state = unsafe { State::from_ptr(l) };
	unsafe {
		SHM = Some(ShmemConf::new().flink("powder-game-2").open().unwrap());
	}
	state.register("read_shm", Some(read_shm));
	1
}
