import { invoke } from "@tauri-apps/api/core";

export async function set(key: string, value: string): Promise<void> {
  await invoke<{ key: string; value: string }>("plugin:os-keystore|set", {
    payload: {
      key,
      value,
    },
  });
}

export async function get(key: string): Promise<string | undefined> {
  return await invoke<{ value?: string }>("plugin:os-keystore|get", {
    payload: {
      key,
    },
  }).then((r) => r.value);
}

export async function remove(key: string): Promise<void> {
  await invoke<{ value?: string }>("plugin:os-keystore|remove", {
    payload: {
      key,
    },
  });
}
