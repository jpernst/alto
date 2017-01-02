extern crate alto;

use std::sync::Arc;
use alto::*;


fn main() {
	let alto = Alto::load_default().unwrap();
	println!("Using output: {:?}", alto.default_output().unwrap());

	let dev = alto.open(None).unwrap();
	let ctx = dev.new_context(None).unwrap();

	{
		let mut buf = ctx.new_buffer().unwrap();
		buf.set_data::<alto::Mono<i16>, Vec<_>>(SinWave::new(44_000 / 440, 0.25).render().take(44_000 / 440).collect(), 44_000).unwrap();
		let buf = Arc::new(buf);

		let mut src = ctx.new_static_source().unwrap();
		src.set_buffer(Some(buf)).unwrap();
		src.set_looping(true).unwrap();

		println!("Playing static 440hz sine wave...");
		src.play().unwrap();

		std::thread::sleep(std::time::Duration::new(2, 0));
	}

	{
		let mut wave = SinWave::new(44_000 / 220, 0.25);

		let mut src = ctx.new_streaming_source().unwrap();
		for _ in 0 .. 5 {
			let mut buf = ctx.new_buffer().unwrap();
			buf.set_data::<alto::Mono<i16>, Vec<_>>(wave.render().take(44_000 / 10).collect(), 44_000).unwrap();
			src.queue_buffer(buf).map_err(|e| e.0).unwrap();
		}

		println!("Playing streaming 220hz sine wave...");
		src.play().unwrap();

		for _ in 0 .. 15 {
			while src.buffers_processed().unwrap() == 0 { }

			let mut buf = src.unqueue_buffer().unwrap();
			buf.set_data::<alto::Mono<i16>, Vec<_>>(wave.render().take(44_000 / 10).collect(), 44_000).unwrap();
			src.queue_buffer(buf).map_err(|e| e.0).unwrap();
		}

		while src.buffers_processed().unwrap() < 5 { }
	}
}


struct SinWave {
	len: i32,
	vol: f32,
	cursor: i32,
}

struct SinWaveRenderer<'w>(&'w mut SinWave);


impl SinWave {
	pub fn new(len: i32, vol: f32) -> SinWave {
		SinWave{len: len, vol: vol, cursor: 0}
	}


	pub fn render(&mut self) -> SinWaveRenderer {
		SinWaveRenderer(self)
	}
}


impl<'w> Iterator for SinWaveRenderer<'w> {
	type Item = i16;

	fn next(&mut self) -> Option<i16> {
		let cursor = self.0.cursor;
		self.0.cursor += 1;
		if self.0.cursor == self.0.len { self.0.cursor = 0 }

		Some(((cursor as f32 / self.0.len as f32 * 2.0 * std::f32::consts::PI).sin() * self.0.vol * std::i16::MAX as f32) as i16)
	}
}
