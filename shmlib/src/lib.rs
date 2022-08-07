struct SharedMemory {
	lock: bool,
	width: usize,
	height: usize,
	data: [u32]
}

pub extern "C" fn hello_world() {
	println!("OwO");
}