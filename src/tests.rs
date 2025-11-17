
use crate::*;

#[test]
fn main() {
	println!("============================== TEST START ======================================");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(start());

	println!("================================ TEST END ====================================");
}

fn setup_socket(
	address: libc::in6_addr,
	port: u16,

	backlog: u32,

	reuse_address: bool,
	ipv6_only: bool,

) -> Result<l3::File, l3::Error> {

	let socket = l3::File::socket(
		l3::AddressFamily::IPV6,
		libc::SOCK_STREAM | libc::SOCK_NONBLOCK,
	)?;

	socket.reuse_address(reuse_address)?;

	socket.ipv6_only(ipv6_only)?;

	let endpoint = libc::sockaddr_in6 {
		sin6_family: libc::AF_INET6 as _,
		sin6_addr: address,
		sin6_port: port.to_be(),
		sin6_flowinfo: 0,
		sin6_scope_id: 0,
	};

	socket.bind(&endpoint)?;

	socket.listen(backlog)?;

	Ok(socket)
}

async fn start() -> Result<(), l3::Error> {

	let socket = setup_socket(
	unsafe { libc::in6addr_any },
		8080,
		256,
		true,
		false,
	)?;

	loop {

		let result = l3::would_block!(
			socket.accept_all(libc::SOCK_NONBLOCK)
		).await;

		match result {
			Err(value) => return Err(value),
			Ok(connection) => handle(
				connection.socket,
				connection.endpoint,
			).await,
		}
	}

	Ok(())
}

async fn handle(socket: l3::File, endpoint: l3::SocketIPV6) {

	let mut buffer = [0 as u8; 1024];

	loop {

		let value = would_block!(
			socket.read(buffer.as_mut_slice())
		).await;

		match value {
			Err(value) => panic!("ERROR"),
			Ok(0) => return,
			Ok(read) => {
				println!("{}", std::str::from_utf8(&buffer[..read]).unwrap());
				break;
			},
		}
	}
	
	socket.write(b"hello world");



	socket.shutdown(l3::ShutdownHow::Write).unwrap();
}
