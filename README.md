# Tauri Plugin os-secrets

Store your secrets in the OS keychain.

Provides a wrapper around the [keyring](https://crates.io/crates/keyring) crate and adds Android support ontop.

Uses encrypted shared preferences on Android.

Tracking the issue of Android support for `keyring` [#127](https://github.com/hwchen/keyring-rs/issues/127). If this is fixed we can
remove the custom Android support.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Usage

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os_secrets::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Access the secrets apis from the `app_handle`

```rust
use tauri::Manager;
use tauri_plugin_os_secrets::OsSecretsExt;

// app is a tauri::AppHandle
app.os_secrets().set("secret-key", "test")?;
let secret: Option<String> = app.os_secrets().get("secret-key")?;
app.os_secrets().remove("secret-key")?;
```
