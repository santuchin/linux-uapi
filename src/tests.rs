

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

use crate::*;
use l3::*;

async fn start() {
	
}
