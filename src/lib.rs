#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod arch;

/*
#[cfg(not(target_os = "linux"))]
compile_error!("linux-syscalls only supports Linux");

*/

pub use arch::current::*;


#[cfg(test)] mod tests;
