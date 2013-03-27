extern mod openal;
use al = openal::hl;

fn main() {
    io::println(al::Device::default_name());
    for al::Device::get_available().each |&s| {
        io::println(fmt!("- %s", s));
    }
    
    io::println(al::CaptureDevice::default_name());
    for al::CaptureDevice::get_available().each |&s| {
        io::println(fmt!("- %s", s));
    }
    
    let device = al::Device::open_default().get();
    let context = al::Context::create(&device).get();
    
    // ...
    
    context.destroy();
    device.close();
}