use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(any(desktop, target_os = "ios"))]
mod desktop;
#[cfg(target_os = "android")]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(any(desktop, target_os = "ios"))]
use desktop::OsSecrets;
#[cfg(target_os = "android")]
use mobile::OsSecrets;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the os-secrets APIs.
pub trait OsSecretsExt<R: Runtime> {
  fn os_secrets(&self) -> &OsSecrets<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OsSecretsExt<R> for T {
  fn os_secrets(&self) -> &OsSecrets<R> {
    self.state::<OsSecrets<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("os-secrets")
    .invoke_handler(tauri::generate_handler![commands::set, commands::get, commands::remove])
    .setup(|app, api| {
      #[cfg(target_os = "android")]
      let os_secrets = mobile::init(app, api)?;
      #[cfg(any(desktop, target_os = "ios"))]
      let os_secrets = desktop::init(app, api)?;
      app.manage(os_secrets);
      Ok(())
    })
    .build()
}
