extern mod openal;
use openal::hl::alc;

fn main() {
    io::println(alc::Device::default_name());
    for alc::Device::get_available().each |&s| {
        io::println(fmt!("- %s", s));
    }
    
    io::println(alc::CaptureDevice::default_name());
    for alc::CaptureDevice::get_available().each |&s| {
        io::println(fmt!("- %s", s));
    }
    
    let device = alc::Device::open_default().get();
    let context = alc::Context::create(&device, []).get();
    
    // ...
    
    context.destroy();
    device.close();
}