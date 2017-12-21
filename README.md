# alto

`alto` provides idiomatic Rust bindings for [OpenAL 1.1](http://connect.creativelabs.com/openal/)
and extensions (including EFX).

## WARNING

Because Alto interacts with global C state via dynamic linking, having multiple versions of Alto in one project could lead to unsafety.
Please make sure only one version of Alto is in your dependency tree at any given time.

## API Usage

```rust
let alto = Alto::load_default()?;

for s in alto.enumerate_outputs() {
    println!("Found device: {}", s.to_str()?);
}

let device = alto.open(None)?; // Opens the default audio device
let context = device.new_context(None)?; // Creates a default context

// Configure listener
context.set_position([1.0, 4.0, 5.0]);
context.set_velocity([2.5, 0.0, 0.0]);
context.set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]));

let source = context.new_static_source()?;

// Now you can load your samples and store them in a buffer with
// `context.new_buffer(samples, frequency)`;
```
