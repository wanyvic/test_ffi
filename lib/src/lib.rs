use {log::info, std::fmt::Debug};

#[derive(Debug)]
pub struct FFIPayload {
  pub data: Vec<u8>,
}

pub trait Plugin: Send + Sync + Debug {
  fn send_payload(&self, payload: Vec<FFIPayload>) -> Result<Vec<FFIPayload>, String>;
}

#[derive(Default, Debug)]
pub struct TestPlugin;

impl Drop for TestPlugin {
  fn drop(&mut self) {
    info!("TestPlugin is being dropped.");
  }
}

impl Plugin for TestPlugin {
  fn send_payload(&self, payload: Vec<FFIPayload>) -> Result<Vec<FFIPayload>, String> {
    info!("TestPlugin received a Payload with {:?}.", payload);
    Ok(vec![
      FFIPayload {
        data: vec![5, 6, 7],
      },
      FFIPayload {
        data: vec![8, 9, 10],
      },
    ])
  }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn Plugin {
  // 初始化日志记录器，如果已经初始化过会被忽略
  let _ = env_logger::try_init();

  info!("Creating TestPlugin instance");
  let plugin = TestPlugin::default();
  let plugin: Box<dyn Plugin> = Box::new(plugin);
  Box::into_raw(plugin)
}
