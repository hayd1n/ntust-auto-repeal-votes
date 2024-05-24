// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::time::sleep;

#[tauri::command]
fn get_url(handle: tauri::AppHandle) -> String {
    let window = handle.get_window("ntust").unwrap();
    window.url().to_string()
}

#[tauri::command]
fn find_courses(handle: tauri::AppHandle) -> String {
    const JS_CODE: &'static str = include_str!("../js/find_courses.js");

    let window = handle.get_window("ntust").unwrap();
    window.eval(JS_CODE).unwrap();

    let url = window.url().to_string();

    url
}

#[tauri::command]
fn do_vote(handle: tauri::AppHandle, course_no: String) {
    const JS_CODE: &'static str = include_str!("../js/do_vote.js");

    println!("do_vote: {}", course_no);

    let window = handle.get_window(course_no.as_str()).unwrap();
    window.eval(JS_CODE).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Course {
    course_no: String,
    course_name: String,
    payload: String,
}

#[tauri::command]
async fn start_votes(
    handle: tauri::AppHandle,
    home_url: String,
    post_url: String,
    courses: Vec<Course>,
) {
    let window = handle.get_window("main").unwrap();

    let monitor = window.current_monitor().unwrap().unwrap();

    let width = monitor.size().width as f64;
    let height = monitor.size().height as f64;
    let scale_factor = monitor.scale_factor();

    let window_width = width / courses.len() as f64 / scale_factor;
    let window_height = height / scale_factor;

    // create windows for each course
    for (i, course) in courses.iter().enumerate() {
        let window = tauri::WindowBuilder::new(
            &handle,
            &course.course_no,
            tauri::WindowUrl::External(home_url.parse().unwrap()),
        )
        .title(format!("{} - {}", course.course_no, course.course_name))
        .inner_size(window_width, window_height)
        .position(window_width * i as f64, 0.0)
        .closable(false)
        .build()
        .unwrap();

        window
            .eval(
                format!(
                    r#"document.addEventListener("DOMContentLoaded", () => post('{}',{}))"#,
                    post_url, &course.payload
                )
                .as_str(),
            )
            .unwrap();
    }

    // waiting for all ready
    loop {
        let all_ready = courses.iter().all(|c| {
            handle
                .get_window(&c.course_no)
                .unwrap()
                .url()
                .to_string()
                .contains(&post_url)
        });

        println!("{}", all_ready);

        if all_ready {
            break;
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }

    sleep(Duration::from_secs(5)).await;

    // do voting
    for (_, course) in courses.iter().enumerate() {
        let window = handle.get_window(&course.course_no).unwrap();

        const JS_CODE: &'static str = include_str!("../js/do_vote.js");

        window.eval(JS_CODE).unwrap();
    }

    handle
        .get_window("main")
        .unwrap()
        .emit("readyToSubmit", {})
        .unwrap();

    ()
}

#[tauri::command]
fn close_courses(handle: tauri::AppHandle, courses: Vec<Course>) {
    for course in courses {
        let window = handle.get_window(&course.course_no).unwrap();
        window.close().unwrap()
    }
}

#[tauri::command]
async fn submit_courses(
    handle: tauri::AppHandle,
    home_url: String,
    courses: Vec<Course>,
    dry_run: bool,
) {
    let js_code = format!(
        "const DRY_RUN = {};\n{}",
        dry_run.to_string(),
        include_str!("../js/submit.js")
    );

    for course in &courses {
        let window = handle.get_window(&course.course_no).unwrap();
        window.eval(js_code.as_str()).unwrap();
    }

    sleep(Duration::from_secs(3)).await;

    // waiting for all submitted
    loop {
        let all_ready = courses.iter().all(|c| {
            handle
                .get_window(&c.course_no)
                .unwrap()
                .url()
                .to_string()
                .contains(&home_url)
        });

        if all_ready {
            break;
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }

    // // close all windows
    // for course in &courses {
    //     let window = handle.get_window(&course.course_no).unwrap();
    //     window.close().unwrap()
    // }

    handle
        .get_window("main")
        .unwrap()
        .emit("submitComplete", {})
        .unwrap();
}

#[tauri::command]
fn redirect_ntust(handle: tauri::AppHandle, url: String) {
    let window = handle.get_window("ntust").unwrap();
    window
        .eval(format!(r#"window.location = "{}""#, url).as_str())
        .unwrap()
}

#[tauri::command]
fn reload_ntust(handle: tauri::AppHandle) {
    let window = handle.get_window("ntust").unwrap();
    window.eval("location.reload();").unwrap()
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                if event.window().label() == "main" {
                    event.window().app_handle().exit(0)
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_url,
            find_courses,
            do_vote,
            start_votes,
            close_courses,
            submit_courses,
            redirect_ntust,
            reload_ntust
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
