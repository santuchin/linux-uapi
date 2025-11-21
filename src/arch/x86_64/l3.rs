
pub use std::ffi::CStr;
use std::ffi::c_int;
use std::ops::Add;

use crate::l2::{self, open};
use crate::result::Error;
pub use crate::types::*;


pub fn pause() -> () {
	l2::pause();
}

pub fn scheduler_yield() -> () {
	l2::scheduler_yield();
}

pub fn get_process_id() -> ProcessId {
	ProcessId { value: l2::get_process_id().value as _ }
}

pub fn exit(status: u8) -> ! {
	unsafe {
		l2::exit(status as _);
		core::hint::unreachable_unchecked()
	}
}




use core::task::{
	Poll,
	Context,
};

use core::pin::Pin;

pub struct WouldBlock<F>(pub F);

impl<F, T> Future for WouldBlock<F>
where
	F: FnMut() -> Result<T, Error>,
{
	type Output = Result<T, Error>;

	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		let this = unsafe { self.get_unchecked_mut() };
		match (this.0)() {
			Err(Error::Again) => Poll::Pending,
			other => Poll::Ready(other),
		}
	}
}

#[macro_export] macro_rules! would_block {
    ($expr:expr) => {
		{
			$crate::l3::WouldBlock(|| { $expr })
		}
    };
}





#[derive(Debug, Clone)]
pub struct FileDesc {
	pub raw: l2::RawFd,
}

impl FileDesc {

	pub fn openat2(
		dir_file_desc: FileDesc,
		filename: &CStr,
		open_how: OpenHow,
	) -> Result<Self, Error> {
		unsafe {
			l2::openat2(
				dir_file_desc.raw,
				filename.as_ptr() as _,
				&open_how,
				core::mem::size_of_val(&open_how),
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	pub fn long_seek(
		&self,
		offset: isize,
		whence: Seek,
	) -> Result<isize, Error> {
		unsafe {
			l2::long_seek(
				self.raw,
				offset as _,
				whence as _,
			).catch()
		}.map(|value| value as isize)
	}

	pub fn open(
		filename: &CStr,
		flags: OpenFlags,
		mode: u32,
	) -> Result<FileDesc, Error> {
		unsafe {
			l2::open(
				filename.as_ptr(),
				flags.raw,
				mode,
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	pub fn read(&self, buffer: &mut [u8]) -> Result<usize, Error> {
		unsafe {
			l2::read(
				self.raw,
				buffer.as_mut_ptr(),
				buffer.len(),
			).catch()
		}.map(|value| value as usize)
	}

	pub fn write(&self, data: &[u8]) -> Result<usize, Error> {
		unsafe {
			l2::write(
				self.raw,
				data.as_ptr(),
				data.len(),
			).catch()
		}.map(|value| value as usize)
	}

	pub fn socket(
		family: AddressFamily,
		semantic: ProtocolSemantic,
		protocol: c_int,
	) -> Result<Self, Error> {
		unsafe {
			l2::socket(
				family as _,
				semantic as _,
				protocol,
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	pub fn set_socket_option<T>(
		&self,
		level: c_int,
		option: c_int,
		value: &T,
	) -> Result<c_int, Error> {
		unsafe {
			l2::set_socket_option(
				self.raw,
				level,
				option,
				value,
				core::mem::size_of_val(&value),
			).catch()
		}.map(|value| value as c_int)
	}

	pub fn listen(&self, backlog: u32) -> Result<(), Error> {
		unsafe {
			l2::listen(
				self.raw,
				backlog as _,
			).catch()
		}.map(|_| ())
	}

	pub fn bind<T>(&self, address: &T) -> Result<(), Error> {
		unsafe {
			l2::bind(
				self.raw,
				address as *const _ as _,
				core::mem::size_of::<T>() as _,
			).catch()
		}.map(|_| ())
	}

	pub fn accept(
		&self,
		non_block: bool,
		close_on_exec: bool,
	) -> Result<Self, Error> {

		let mut flags = 0;
		
		if non_block {
			flags |= libc::SOCK_NONBLOCK;
		}

		if close_on_exec {
			flags |= libc::SOCK_CLOEXEC;
		}

		unsafe {
			l2::accept4(
				self.raw,
				core::ptr::null_mut(),
				core::ptr::null_mut(),
				0,
			).catch()
		}.map(|value| Self { raw: value as _ })
	}

	pub fn accept_with_address(
		&self,
		non_block: bool,
		close_on_exec: bool,
	) -> Result<(Self, libc::sockaddr_storage), Error> {

		let mut endpoint = core::mem::MaybeUninit::<libc::sockaddr_storage>::uninit();
		let mut length = core::mem::size_of_val(&endpoint);

		let mut flags = 0;
		
		if non_block {
			flags |= libc::SOCK_NONBLOCK;
		}

		if close_on_exec {
			flags |= libc::SOCK_CLOEXEC;
		}

		unsafe {
			l2::accept4(
				self.raw,
				&mut endpoint as *mut _ as _,
				&mut length as *mut _ as _,
				flags,
			).catch()
		}.map(|value|
			(
				Self { raw: value as _},
				unsafe { endpoint.assume_init() }
			)
		)
	}



	pub fn accept_with_address_paranoid(
		&self,
		non_block: bool,
		close_on_exec: bool,
	) -> Result<(Self, Result<libc::sockaddr_storage, ()>), Error> {

		let mut endpoint = core::mem::MaybeUninit::<libc::sockaddr_storage>::uninit();
		let mut length = core::mem::size_of_val(&endpoint);

		let mut flags = 0;
		
		if non_block {
			flags |= libc::SOCK_NONBLOCK;
		}

		if close_on_exec {
			flags |= libc::SOCK_CLOEXEC;
		}

		unsafe {
			l2::accept4(
				self.raw,
				&mut endpoint as *mut _ as _,
				&mut length as *mut _ as _,
				flags,
			).catch()
		}.map(|value|
			(
				Self { raw: value as _},
				if length < core::mem::size_of::<libc::sa_family_t>() {
					Err(())
				} else {
					Ok(unsafe { endpoint.assume_init() })
				},
			)
		)
	}

	pub fn close(self) -> Result<(), Error> {
		unsafe {
			l2::close(self.raw).catch()
		}.map(|_| ())
	}
	

	pub fn setup_socket_test(address: u128, port: u16, backlog: u32) -> Result<FileDesc, Error> {

		let socket = FileDesc::socket(
			AddressFamily::IPV6,
			ProtocolSemantic::Stream,
			0
		)?;

		socket.set_socket_option(
			libc::SOL_SOCKET,
			libc::SO_REUSEADDR,
			&(true as c_int),
		)?;

		let address = libc::sockaddr_in6 {
			sin6_family: libc::AF_INET6 as _,
			sin6_addr: libc::in6_addr { s6_addr: address.to_ne_bytes() },
			sin6_port: port.to_be(),
			sin6_flowinfo: 0,
			sin6_scope_id: 0,
		};

		socket.bind(&address)?;

		socket.listen(backlog)?;

		Ok(socket)
	}
}

impl Drop for FileDesc {
	fn drop(&mut self) {
		unsafe {
			l2::close(self.raw)
		};
	}
}

pub static STD_INPUT: FileDesc = FileDesc { raw: 0 };
pub static STD_OUTPUT: FileDesc = FileDesc { raw: 1 };
pub static STD_ERROR: FileDesc = FileDesc { raw: 2 };
pub static CURRENT_WORKING_DIRECTORY: FileDesc = FileDesc { raw: -100 };
