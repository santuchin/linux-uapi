

#[macro_export] macro_rules! syscall {

	(
		$number:expr
		
		$(,$a:expr
		$(,$b:expr
		$(,$c:expr
		$(,$d:expr
		$(,$e:expr
		$(,$f:expr
		)?)?)?)?)?)?
		$(,)?
	) => {
		{
			use core::ffi::c_long;

			let value: c_long;

			core::arch::asm!(
				"syscall",
				
				in("rax") ($number) as c_long,
				
				$(in("rdi") ($a),
				$(in("rsi") ($b),
				$(in("rdx") ($c),
				$(in("r10") ($d),
				$(in("r8")  ($e),
				$(in("r9")  ($f),
				)?)?)?)?)?)?

				lateout("rax") value,
				lateout("rcx") _,
				lateout("r11") _,

				options(nostack),
			);

			value
		}
	}
}

pub use syscall;

