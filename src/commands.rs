use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::OsSecretsExt;
use crate::Result;

#[command]
pub(crate) async fn set<R: Runtime>(
    app: AppHandle<R>,
    payload: SetRequest,
) -> Result<()> {
    app.os_secrets().set(&payload.key, &payload.value)
}

#[command]
pub(crate) async fn get<R: Runtime>(
    app: AppHandle<R>,
    payload: GetRequest,
) -> Result<GetResponse> {
    Ok(GetResponse {
        value: app.os_secrets().get(&payload.key)?
    })
}

#[command]
pub(crate) async fn remove<R: Runtime>(
    app: AppHandle<R>,
    payload: RemoveRequest,
) -> Result<()> {
    app.os_secrets().remove(&payload.key)
}
