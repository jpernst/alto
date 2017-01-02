extern crate alto;

use std::sync::Arc;
use alto::DeviceTrait;


fn main() {
	let alto = alto::Alto::load_default().unwrap();
	println!("Using output: {:?}", alto.default_output().unwrap());

	let dev = alto.open(None).unwrap();
	let ctx = dev.new_context(None).unwrap();

	let buf = Arc::new(ctx.new_buffer().unwrap());
	let mut src = ctx.new_static_source().unwrap();

	src.set_buffer(Some(buf));
}
