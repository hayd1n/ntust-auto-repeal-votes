import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from "@tauri-apps/api/window";

export async function resizeBig() {
  const window = getCurrentWebviewWindow();
  await window.setSize(new LogicalSize(800, 800));
}

export async function resizeDefault() {
  const window = getCurrentWebviewWindow();
  await window.setSize(new LogicalSize(800, 600));
}
