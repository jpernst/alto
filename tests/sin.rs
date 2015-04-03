
extern crate openal;

use std::i16;
use std::f64::consts::PI;
use std::thread::sleep_ms;
use openal::al;
use openal::alc;

#[test]
fn play_sin() {
  let device = alc::Device::open(None).expect("Could not open device");
  let ctx = device.create_context(&[]).expect("Could not create context");
  ctx.make_current();

  let buffer = al::Buffer::gen();
  let source = al::Source::gen();

  let sample_freq = 44100.0;
  let tone_freq = 440.0;
  let duration = 3.0;
  let num_samples = (sample_freq * duration) as usize;

  let samples: Vec<i16> = (0..num_samples).map(|x| {
    let t = x as f64;
    ((tone_freq * t * 2.0 * PI / sample_freq).sin() * (i16::MAX - 1) as f64) as i16
  }).collect();

  unsafe { buffer.buffer_data(al::Format::Mono16, &samples, sample_freq as al::ALsizei) };

  source.queue_buffer(&buffer);
  source.play();

  sleep_ms((duration * 1000.0) as u32);

  ctx.destroy();
  device.close().ok().expect("Unable to close device");
}
