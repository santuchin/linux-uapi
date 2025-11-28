
// cargo test -- --nocapture
#![allow(warnings)] #[cfg(test)] mod tests;


const _: () = assert!(std::mem::size_of::<usize>() == std::mem::size_of::<libc::size_t>());
#[cfg(not(target_os = "linux"))] compile_error!("linux-syscalls only supports Linux");


mod arch;

#[cfg(any(target_arch = "x86_64"))] pub use arch::x86_64::*;
