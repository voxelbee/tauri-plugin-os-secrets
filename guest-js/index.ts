import { invoke } from "@tauri-apps/api/core";

export async function store(key: string, value: string): Promise<void> {
  await invoke<{ key: string; value: string }>("plugin:os-keystore|store", {
    payload: {
      key,
      value,
    },
  });
}

export async function load(key: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:os-keystore|load", {
    payload: {
      key,
    },
  }).then((r) => (r.value ? r.value : null));
}
