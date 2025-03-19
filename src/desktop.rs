use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use keyring::{Entry, Error};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<OsKeystore<R>> {
  Ok(OsKeystore(app.clone()))
}

/// Access to the os-keystore APIs.
pub struct OsKeystore<R: Runtime>(AppHandle<R>);

impl<R: Runtime> OsKeystore<R> {
  pub fn store(&self, payload: StoreRequest) -> crate::Result<()> {
    Entry::new(&self.0.package_info().name, &payload.key)
      .map_err(|e| crate::Error::PlatformFailure(Box::new(e)))?
      .set_secret(payload.value.as_bytes())
      .map_err(|e| crate::Error::PlatformFailure(Box::new(e)))?;

    Ok(())
  }

  pub fn load(&self, payload: LoadRequest) -> crate::Result<LoadResponse> {
    let secret = Entry::new(&self.0.package_info().name, &payload.key)
    .map_err(|e| match e {
      Error::NoEntry => crate::Error::NoEntry,
      e => crate::Error::PlatformFailure(Box::new(e))
    })?
    .get_secret()
    .map_err(|e| match e {
      Error::NoEntry => crate::Error::NoEntry,
      e => crate::Error::PlatformFailure(Box::new(e))
    })?;

    Ok(LoadResponse {
      value: String::from_utf8(secret.to_vec()).unwrap(),
    })
  }
}
