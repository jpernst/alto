extern crate alto;

use alto::DeviceTrait;


fn main() {
	let alto = alto::Alto::load_default().unwrap();
	println!("{:?}", alto.enumerate_outputs());

	let dev = alto.open(None).unwrap();
	println!("HRTFs: {:?}", dev.enumerate_soft_hrtfs());
	println!("HRTF: {:?}", dev.soft_hrtf_status());
}
