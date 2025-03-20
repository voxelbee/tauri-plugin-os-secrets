use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use keyring::{Entry, Error};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<OsSecrets<R>> {
  Ok(OsSecrets(app.clone()))
}

/// Access to the os-secrets APIs.
pub struct OsSecrets<R: Runtime>(AppHandle<R>);

impl<R: Runtime> OsSecrets<R> {
  /// Stores a new value in the OS store or overwrites an exisiting value
  pub fn set(&self, key: &str, value: &str) -> crate::Result<()> {
    Entry::new(&self.0.package_info().name, key)
      .map_err(|e| crate::Error::PlatformFailure(Box::new(e)))?
      .set_secret(value.as_bytes())
      .map_err(|e| crate::Error::PlatformFailure(Box::new(e)))?;

    Ok(())
  }

  /// Tries to get a secret from the keyring returns [`None`] if there is no secret.
  pub fn get(&self, key: &str) -> crate::Result<Option<String>> {
    let secret = Entry::new(&self.0.package_info().name, key)
    .map_err(|e| crate::Error::PlatformFailure(Box::new(e)))?
    .get_secret();

    match secret {
      Ok(secret) => Ok(Some(String::from_utf8(secret.to_vec()).unwrap())),
      Err(e) => match e {
        Error::NoEntry => Ok(None),
        e => Err(crate::Error::PlatformFailure(Box::new(e)))
        }
    }
  }

  /// Tries to delete the secret. Does nothing is there is no secret.
  pub fn remove(&self, key: &str) -> crate::Result<()> {
    let secret = Entry::new(&self.0.package_info().name, key)
    .map_err(|e| crate::Error::PlatformFailure(Box::new(e)))?
    .delete_credential();

    match secret {
      Ok(_) => Ok(()),
      Err(e) => match e {
        Error::NoEntry => Ok(()),
        e => Err(crate::Error::PlatformFailure(Box::new(e)))
        }
    }
  }
}
