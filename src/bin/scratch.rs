extern crate alto;

use alto::DeviceTrait;


fn main() {
	let alto = alto::Alto::load_default().unwrap();
	println!("Using output: {:?}", alto.default_output().unwrap());

	let dev = alto.open(None).unwrap();
	let ctx = dev.new_context(None);
}
