use anyhow::Result;
use tauri::Emitter;

use crate::speedtest::config::{init_data_dir, DEFAULT_SUB_URL};
use crate::speedtest::task::run_task_android;

pub async fn run(workers: usize, top: usize, window: tauri::Window) -> Result<()> {
    let output = crate::m3u::m3u_path()?;
    let data_dir = output.parent().unwrap().to_path_buf();
    init_data_dir(Some(&data_dir));

    let urls = vec![DEFAULT_SUB_URL.to_string()];
    let win = window.clone();
    let _ = win.emit("speedtest://progress", "正在启动测速…");

    let output_clone = output.clone();
    let window_clone = window.clone();

    tokio::spawn(async move {
        // Progress monitor: emit phase messages every few seconds
        let win2 = window_clone.clone();
        let out2 = output_clone.clone();
        let monitor = tokio::spawn(async move {
            let phases = [
                "正在获取频道列表…",
                "正在测速 API 源…",
                "正在测速订阅源…",
                "整理频道中…",
            ];
            for phase in phases {
                tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
                let _ = win2.emit("speedtest://progress", phase);
            }
            // After phases keep pulsing
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                let size = std::fs::metadata(&out2).map(|m| m.len()).unwrap_or(0);
                if size > 0 {
                    let _ = win2.emit("speedtest://progress", "即将完成…");
                    break;
                }
            }
        });

        let count = run_task_android(workers, top, urls, &output_clone).await;
        monitor.abort();

        if count == 0 {
            let _ = window_clone.emit("speedtest://error", "未找到可用频道，请检查网络后重试");
        } else {
            let _ = window_clone.emit("speedtest://done", count);
        }
    });

    Ok(())
}
