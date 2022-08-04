https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

# Making crates produce dlls for C programs
## Cargo.toml
https://github.com/rkarp/rust-dll-demo/blob/9c830e141b4a4f638cee6b129196eb7f3b5cce93/Cargo.toml#L11
```toml
[lib]
crate-type = ["cdylib"]
```

## lib.rs
```rust
#[no_mangle]
pub extern fn hello() {
}
```

## main.rs
```rust
use libloading::{Library, Symbol};

fn main() {
    unsafe {
        let lib = Library::new("lib.dll").unwrap();
        let func: Symbol<unsafe fn()> = lib.get(b"hello").unwrap();
        func();
    }
}
```

# Shared Memory
