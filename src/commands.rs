use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::OsKeystoreExt;

#[command]
pub(crate) async fn store<R: Runtime>(
    app: AppHandle<R>,
    payload: StoreRequest,
) -> Result<()> {
    app.os_keystore().store(payload)
}

#[command]
pub(crate) async fn load<R: Runtime>(
    app: AppHandle<R>,
    payload: LoadRequest,
) -> Result<LoadResponse> {
    app.os_keystore().load(payload)
}
