extern crate alto;

use alto::{Alto, AltoResult};

fn run() -> AltoResult<()> {
    let alto = Alto::load_default()?;

    for s in alto.enumerate_outputs() {
        println!("Found device: {}", s.to_str().unwrap());
    }

    let device = alto.open(None)?; // Opens the default audio device
    let context = device.new_context(None)?; // Creates a default context

    // Configure listener
    context.set_position([1.0, 4.0, 5.0])?;
    context.set_velocity([2.5, 0.0, 0.0])?;
    context.set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))?;

    let _source = context.new_static_source()?;

    // Now you can load your samples and store them in a buffer with
    // `context.new_buffer(samples, frequency)`;

    Ok(())
}

fn main() {
    use std::process::exit;

    if let Err(e) = run() {
        println!("Failed to run basic example: {}", e);
        exit(1);
    }
}
