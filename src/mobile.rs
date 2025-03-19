use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_os_keystore);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<OsKeystore<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_os_keystore)?;
  Ok(OsKeystore(handle))
}

/// Access to the os-keystore APIs.
pub struct OsKeystore<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> OsKeystore<R> {
  pub fn store(&self, payload: StoreRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("store", payload)
      .map_err(Into::into)
  }

  pub fn load(&self, payload: LoadRequest) -> crate::Result<LoadResponse> {
    self
      .0
      .run_mobile_plugin("load", payload)
      .map_err(Into::into)
  }
}
