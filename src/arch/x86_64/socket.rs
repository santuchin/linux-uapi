
use std::{ffi::c_uint};

use crate::{
	l3,
	l2
};

pub struct SocketIPV6 {
	file_desc: l3::FileDesc,
}


pub struct SocketAddressIPV6 {
	pub address: u128,
	pub port: u16,
	pub flowinfo: u32,
	pub scope_id: u32,
}

impl SocketIPV6 {

	pub fn new(
		semantic: l3::ProtocolSemantic
	) -> l3::Result<Self> {
		l3::FileDesc::socket(
			l3::AddressFamily::IPV6,
			semantic,
			0,
		).map(|value| Self { file_desc: value })
	}

	pub fn bind(
		&self,
		address: &SocketAddressIPV6,
	) -> l3::Result<()> {
		self.file_desc.bind(
			&libc::sockaddr_in6 {
				sin6_family: libc::AF_INET6 as _,
				sin6_addr: libc::in6_addr { s6_addr: address.address.to_ne_bytes() },
				sin6_port: address.port.to_be(),
				sin6_flowinfo: address.flowinfo,
				sin6_scope_id: address.scope_id,
			}
		)
	}

	pub fn listen(&self, backlog: u32) -> l3::Result<()> {
		self.file_desc.listen(backlog)
	}

	pub fn accept(
		&self, 
		non_block: bool,
		close_on_exec: bool,
	) -> l3::Result<(l3::FileDesc, SocketAddressIPV6)> {

		self.file_desc.accept_with_address::<libc::sockaddr_in6>(non_block, close_on_exec).map(
			|value| (
				value.0,
				SocketAddressIPV6 {
					address: u128::from_be_bytes(value.1.sin6_addr.s6_addr),
					port: value.1.sin6_port,
					flowinfo: value.1.sin6_flowinfo,
					scope_id: value.1.sin6_scope_id,
				},
			)
		)
	}
}