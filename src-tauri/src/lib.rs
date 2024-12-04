use std::{env, fs, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Listener, Manager, Url, UserAttentionType, WebviewWindow};

const NTUST_JUDGE_URL: &str = "https://stuinfosys.ntust.edu.tw/JudgeCourseServ/JudgeCourse";
const NTUST_JUDGE_POST_URL: &str =
    "https://stuinfosys.ntust.edu.tw/JudgeCourseServ/JudgeQuestion/ListJudge";
const LOGOUT_URL: &str = "https://stuinfosys.ntust.edu.tw/NTUSTSSOServ/SSO/Logout/JudgeCourseServ";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Course {
    course_name: String,
    course_no: String,
    payload: String,
}

enum AppState {
    Standby,
    Voting,
}

fn inject_js(window: WebviewWindow) {
    #[cfg(debug_assertions)]
    {
        let path = env::current_dir().unwrap();
        println!("{:?}", path.join("../inject/index.js"));
        window
            .eval(
                fs::read_to_string(path.join("../inject/index.js"))
                    .unwrap()
                    .as_str(),
            )
            .unwrap();
    }
    #[cfg(not(debug_assertions))]
    window.eval(include_str!("../../inject/index.js")).unwrap();

    // Run the main function in the injected script
    window.eval("injectMain()").unwrap();
}

#[tauri::command]
fn show_login_window(handle: AppHandle) {
    let mut window = handle.get_webview_window("ntust").unwrap();
    window
        .navigate(Url::parse(NTUST_JUDGE_URL).unwrap())
        .unwrap();
    window.center().unwrap();
    window.show().unwrap();
    window
        .request_user_attention(Some(UserAttentionType::Informational))
        .unwrap();
}

#[tauri::command]
fn hide_login_window(handle: AppHandle) {
    let window = handle.get_webview_window("ntust").unwrap();
    window.hide().unwrap();
}

#[tauri::command]
fn logout(handle: AppHandle) {
    let mut window = handle.get_webview_window("ntust").unwrap();
    window.navigate(LOGOUT_URL.parse().unwrap()).unwrap();
    window.clear_all_browsing_data().unwrap();
}

#[tauri::command]
fn refresh_info(handle: AppHandle) {
    let mut window = handle.get_webview_window("ntust").unwrap();
    window
        .navigate(Url::parse(NTUST_JUDGE_URL).unwrap())
        .unwrap();
}

#[tauri::command]
fn voting(handle: AppHandle, courses: Vec<Course>) {
    let mut url = Url::parse(NTUST_JUDGE_URL).unwrap();
    url.set_fragment(Some("voting"));
    let voting_window =
        tauri::WebviewWindowBuilder::new(&handle, "voting", tauri::WebviewUrl::External(url))
            .disable_drag_drop_handler()
            .title("Voting")
            .maximizable(false)
            .inner_size(1024.0, 800.0)
            .build()
            .unwrap();
    if let Some(c) = courses.get(0) {
        let js = format!(
            r#"
            document.addEventListener("DOMContentLoaded", () => {{
                post('{}', {})
            }});
            "#,
            NTUST_JUDGE_POST_URL, c.payload
        );
        println!("{}", js);
        voting_window.eval(&js).unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            show_login_window,
            hide_login_window,
            logout,
            refresh_info,
            voting
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::Standby));

            let handle = app.handle();
            let _ntust_window = tauri::WebviewWindowBuilder::new(
                handle,
                "ntust",
                tauri::WebviewUrl::External(NTUST_JUDGE_URL.parse().unwrap()),
            )
            .disable_drag_drop_handler()
            .title("NTUST Web")
            .maximizable(false)
            .inner_size(1024.0, 800.0)
            .visible(false)
            .on_page_load(|window, _payload| {
                inject_js(window);
            })
            .build()
            .unwrap();

            Ok(())
        })
        .on_window_event(|window, event| match window.label() {
            "ntust" => match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
