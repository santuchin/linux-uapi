

#[test]
fn main() {
	println!("============================== TEST START ==================================== ");

	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	runtime.block_on(start());

	println!("================================ TEST END ====================================");
}

use std::{ffi::{CString, c_int}, fs::File};

use crate::*;
use l3::*;
use libc::{SOCK_NONBLOCK, sockaddr_in6};


async fn start() {

	let socket = FileDesc::setup_socket_test(
		0,
		8080,
		256,
	).unwrap();

	loop {

		let result = would_block!(socket.accept(true, false)).await;

		match result {
			Err(_) => panic!(),
			Ok(connection) => handle(connection).await,
		}
	}

}

async fn handle(connection: FileDesc) {

	use tokio::time;

	let status = 200;
	let content = include_str!("tests.rs");

	let message = format!(
		"HTTP/1.1 {}\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
		status,
		content.len(),
		content,
	);

	match would_block!(connection.write(message.as_bytes())).await {
		Err(_) => return,
		Ok(_) => (),
	}

}
