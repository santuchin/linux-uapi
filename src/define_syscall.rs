
#[macro_export] macro_rules! define_syscall {
	(
		$syscall:expr,
		$main:expr,
		[$($input:expr),* $(,)?],
		[$($clobber:expr),* $(,)?]
		$(,)?
	) => {

		#[macro_export] macro_rules! syscall {

			(
				$number:expr
				
				$(,$arg_0:expr
				$(,$arg_1:expr
				$(,$arg_2:expr
				$(,$arg_3:expr
				$(,$arg_4:expr
				$(,$arg_5:expr
				)?)?)?)?)?)?

				$(,)?
			) => {
				{
					use core::ffi::c_long;

					let value: c_long;

					core::arch::asm!(
						$syscall,
						
						in($main) ($number) as c_long,
					
						$(in("rdi") ($a),
						$(in("rsi") ($b),
						$(in("rdx") ($c),
						$(in("r10") ($d),
						$(in("r8")  ($e),
						$(in("r9")  ($f),
						)?)?)?)?)?)?

						lateout($main) value,

						$( lateout($clobber) _ )*

						options(nostack),
					);

					value
				}
			}
		}

		pub use syscall;

	};
}

/*
define_syscall!(
	"syscall",
	"rax",
	["rdi", "rsi", "rdx", "r10", "r8", "r9"],
	["rcx", "r11"],
);
*/
