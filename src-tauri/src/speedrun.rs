use anyhow::Result;
use tauri::Emitter;

use crate::speedtest::config::{init_data_dir, DEFAULT_SUB_URL};
use crate::speedtest::task::run_task_with_progress;

pub async fn run(workers: usize, top: usize, window: tauri::Window) -> Result<()> {
    let output = crate::m3u::m3u_path()?;
    let data_dir = output.parent().unwrap().to_path_buf();
    init_data_dir(Some(&data_dir));

    let urls = vec![DEFAULT_SUB_URL.to_string()];
    let win = window.clone();
    let _ = win.emit("speedtest://progress", serde_json::json!({
        "phase": "正在启动测速引擎…",
        "detail": "",
        "percent": 0
    }));

    let output_clone = output.clone();
    let window_clone = window.clone();

    tokio::spawn(async move {
        let win = window_clone.clone();

        let count = run_task_with_progress(
            workers,
            top,
            urls,
            &output_clone,
            move |msg| {
                // Map known messages to percent estimates
                let percent = if msg.contains("获取 API") || msg.contains("订阅文件") {
                    10
                } else if msg.contains("测速") && msg.contains("API") {
                    30
                } else if msg.contains("筛选出") {
                    50
                } else if msg.contains("订阅源频道") {
                    60
                } else if msg.contains("订阅源筛选完成") {
                    75
                } else if msg.contains("候选频道") {
                    85
                } else if msg.contains("写入") {
                    95
                } else if msg.contains("测速完成") {
                    100
                } else {
                    -1 // don't update percent
                };

                let _ = win.emit("speedtest://progress", serde_json::json!({
                    "phase": msg,
                    "percent": percent
                }));
            },
        ).await;

        if count == 0 {
            let _ = window_clone.emit("speedtest://error", "未找到可用频道，请检查网络后重试");
        } else {
            let _ = window_clone.emit("speedtest://done", count);
        }
    });

    Ok(())
}
