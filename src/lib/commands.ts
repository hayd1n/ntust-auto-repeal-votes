import { invoke } from "@tauri-apps/api/core";
import type { Course } from "./course";

export async function showLoginWindow() {
  return invoke<void>("show_login_window");
}

export async function hideLoginWindow() {
  return invoke<void>("hide_login_window");
}

export async function logout() {
  return invoke<void>("logout");
}

export async function refreshInfo() {
  return invoke<void>("refresh_info");
}

export async function voting(courses: Course[]) {
  return invoke<void>("voting", { courses });
}
