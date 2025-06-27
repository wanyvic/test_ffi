use {
  lib::{FFIPayload, Plugin},
  libloading::{Library, Symbol},
  log::info,
};

use jemallocator::Jemalloc;

static GLOBAL: Jemalloc = Jemalloc;
type PluginConstructor = unsafe fn() -> *mut dyn Plugin;

fn main() {
  env_logger::init();

  info!("Starting Test FFI...");
  let lib_path = "./target/debug/liblib.dylib";

  let plugin = unsafe {
    let lib = Library::new(lib_path).expect("Could not load the library.");

    let constructor: Symbol<PluginConstructor> = lib
      .get(b"_create_plugin")
      .expect("Could not find constructor in the library.");

    Box::from_raw(constructor())
  };

  info!("Plugin loaded successfully: {:?}", plugin);

  for i in 0..1000 {
    info!("Processing instruction: {}", i);
    let payloads = plugin
      .send_payload(vec![
        FFIPayload {
          data: vec![1, 2, 3],
        },
        FFIPayload {
          data: vec![4, 5, 6],
        },
      ])
      .expect("Could not send a message to the spawned thread.");

    info!("Payloads: {:?}", payloads);
  }

  info!("Finished invoking the spawned thread.");
}
