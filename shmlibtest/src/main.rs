use libloading::{Library, Symbol};

type HelloFunc = unsafe fn();

fn main() {
	unsafe {
		let lib = Library::new("shmlib.dll").unwrap();
		let func: Symbol<HelloFunc> = lib.get(b"hello_world").unwrap();
		func();
	}
}
