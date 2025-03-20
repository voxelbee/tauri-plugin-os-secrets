use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

// Only inits for andriod as we use the iOS functionality of keyring
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<OsSecrets<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "ExamplePlugin")?;
  Ok(OsSecrets(handle))
}

/// Access to the os-secrets APIs.
pub struct OsSecrets<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> OsSecrets<R> {
  pub fn set(&self, payload: SetRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("set", payload)
      .map_err(Into::into)
  }

  pub fn get(&self, payload: GetRequest) -> crate::Result<GetResponse> {
    self
      .0
      .run_mobile_plugin("get", payload)
      .map_err(Into::into)
  }

  pub fn remove(&self, payload: RemoveRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("remove", payload)
      .map_err(Into::into)
  }
}
