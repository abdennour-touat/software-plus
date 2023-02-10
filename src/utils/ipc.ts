import { invoke } from "@tauri-apps/api";
import { deepFreeze } from "utils-min";

/**
 * Small wrapper on top of tauri api invoke
 *
 * best-practice: Light and narrow external api abstraction.
 */
export interface result<T> {
  data: T;
}
export interface IpcResponse<T> {
  error: {
    message: string;
  };
  result: result<T>;
}
export async function ipc_invoke(
  method: string,
  params?: object
): Promise<any> {
  const response: any = await invoke(method, { params });
  if (response.error != null) {
    console.log("ERROR - ipc_invoke - ipc_invoke error", response);
    throw new Error(response.error.message);
  } else {
    return deepFreeze(response.result);
  }
}
