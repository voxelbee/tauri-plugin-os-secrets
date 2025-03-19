use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::OsKeystore;
#[cfg(mobile)]
use mobile::OsKeystore;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the os-keystore APIs.
pub trait OsKeystoreExt<R: Runtime> {
  fn os_keystore(&self) -> &OsKeystore<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OsKeystoreExt<R> for T {
  fn os_keystore(&self) -> &OsKeystore<R> {
    self.state::<OsKeystore<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("os-keystore")
    .invoke_handler(tauri::generate_handler![commands::store, commands::load])
    .setup(|app, api| {
      #[cfg(mobile)]
      let os_keystore = mobile::init(app, api)?;
      #[cfg(desktop)]
      let os_keystore = desktop::init(app, api)?;
      app.manage(os_keystore);
      Ok(())
    })
    .build()
}
