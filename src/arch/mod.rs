
#[cfg(not(any(
		target_arch = "x86_64",
		target_arch = "aarch64",
	)))
]
compile_error!("linux-syscalls doesn't support your target architecture");

pub mod syscall;

pub mod x86_64;
