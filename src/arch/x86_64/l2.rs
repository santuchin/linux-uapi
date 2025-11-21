
#![allow(dead_code)]
#![allow(unused_imports)]

pub use std::os::fd::RawFd;

use crate::l1::{
	syscall,
};
use crate::Sys;
use crate::types;
use crate::result::Result;


use libc::{

	c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void, mode_t, size_t, socklen_t, ssize_t
};



/*
SAFE SYSCALLS
*/

pub fn pause() -> Result {
	unsafe { syscall!(Sys::Pause) }.into()
}

pub fn scheduler_yield() -> Result {
	unsafe { syscall!(Sys::SchedulerYield) }.into()
}

pub fn get_process_id() -> Result {
	unsafe { syscall!(Sys::GetProcessId) }.into()
}

pub fn get_parent_process_id() -> Result {
	unsafe { syscall!(Sys::GetParentProcessId) }.into()
}

pub fn get_group_id() -> Result {
	unsafe { syscall!(Sys::GetGroupId) }.into()
}

pub fn get_user_id() -> Result {
	unsafe { syscall!(Sys::GetUserId) }.into()
}

pub fn get_thread_id() -> Result {
	unsafe { syscall!(Sys::GetThreadId) }.into()
}

/*
UNSAFE SYSCALLS
*/

pub unsafe fn read<T>(
	file_desc: RawFd,
	buffer: *mut T,
	size: usize,
) -> Result {
	unsafe {
		syscall!(
			Sys::Read,
			file_desc,
			buffer,
			size,
		)
	}.into()
}


pub unsafe fn accept(
	file_desc: RawFd,
	address: *mut libc::sockaddr,
	address_length: *mut socklen_t,
) -> Result {
	syscall!(
		Sys::Accept,
		file_desc,
		address,
		address_length,
	).into()
}

pub unsafe fn accept4(
	file_desc: RawFd,
	address: *mut libc::sockaddr,
	address_length: *mut libc::socklen_t,
	flags: c_int,
) -> Result {
	syscall!(
		Sys::Accept4,
		file_desc,
		address,
		address_length,
		flags,
	).into()
}

pub unsafe fn bind(
	file_desc: RawFd,
	address: *const libc::sockaddr,
	address_length: libc::socklen_t,
) -> Result {
	syscall!(
		Sys::Bind,
		file_desc,
		address,
		address_length,
	).into()
}

pub unsafe fn listen(
	file_desc: RawFd,
	backlog: c_int,
) -> Result {
	syscall!(
		Sys::Listen,
		file_desc,
		backlog,
	).into()
}

pub unsafe fn socket(
	family: c_int,
	semantics: c_int,
	protocol: c_int,
) -> Result {
	syscall!(
		Sys::Socket,
		family,
		semantics,
		protocol
	).into()
}

pub unsafe fn write<T>(
	file_desc: RawFd,
	data: *const T,
	length: size_t,
) -> Result {
	syscall!(
		Sys::Write,
		file_desc,
		data,
		length,
	).into()
}

pub unsafe fn close(file_desc: c_int) -> Result {
	syscall!(Sys::Close, file_desc).into()
}


pub unsafe fn set_socket_option<T>(
	file_desc: c_int,
	level: c_int,
	option: c_int,
	value: *const T,
	option_length: usize,
) -> Result {
	syscall!(
		Sys::SetSocketOption,
		file_desc,
		level,
		option,
		value,
		option_length,
	).into()
}

pub unsafe fn exit(status: c_int) -> Result {
	syscall!(Sys::Exit, status).into()
}

pub unsafe fn long_seek(
	file_desc: RawFd,
	offset: libc::off_t,
	whence: c_uint,
) -> Result {
	syscall!(
		Sys::LongSeek,
		offset,
		whence,
	).into()
}

pub unsafe fn open(
	filename: *const c_char,
	flags: c_int,
	mode: mode_t,
) -> Result {
	syscall!(
		Sys::Open,
		filename,
		flags,
		mode,
	).into()
}

pub unsafe fn openat(
	dir_file_desc: RawFd,
	filename: *const c_char,
	flags: c_int,
	mode: mode_t,
) -> Result {
	syscall!(
		Sys::OpenAt,
		dir_file_desc,
		filename,
		flags,
		mode,
	).into()
}

pub unsafe fn openat2(
	dir_file_desc: RawFd,
	filename: *const char,
	open_how: *const types::OpenHow,
	size: size_t,
) -> Result {
	syscall!(
		Sys::OpenAt,
		dir_file_desc,
		filename,
		open_how,
		size,
	).into()
}

