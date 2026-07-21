
use std::fs::OpenOptions;
use std::io::Write;

fn log_to_file(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("updater.log")
    {
        let _ = writeln!(file, "[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log_to_file("应用启动");
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // 启动时自动检查更新
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // 等待2秒让应用完全启动
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                log_to_file("开始检查更新");
                log_to_file(&format!("当前版本: {}", env!("CARGO_PKG_VERSION")));

                // 使用 tauri-plugin-updater 的 API 检查更新
                use tauri_plugin_updater::UpdaterExt;
                match handle.updater() {
                    Ok(updater) => {
                        log_to_file("updater 获取成功，开始检查...");
                        match updater.check().await {
                            Ok(update) => {
                                log_to_file("检查完成");
                                if let Some(update) = update {
                                    log_to_file(&format!("发现新版本: {}", update.version));
                                    // 有更新，下载并安装
                                    log_to_file("开始下载更新...");
                                    match update.download_and_install(|_, _| {}, || {}).await {
                                        Ok(_) => {
                                            log_to_file("下载完成，准备重启...");
                                            handle.restart();
                                        }
                                        Err(e) => {
                                            log_to_file(&format!("下载失败: {}", e));
                                        }
                                    }
                                } else {
                                    log_to_file("已是最新版本");
                                }
                            }
                            Err(e) => {
                                log_to_file(&format!("检查更新失败: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        log_to_file(&format!("获取 updater 失败: {}", e));
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
