
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // 启动时自动检查更新
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // 等待2秒让应用完全启动
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                println!("[Updater] 开始检查更新...");
                // 使用 tauri-plugin-updater 的 API 检查更新
                use tauri_plugin_updater::UpdaterExt;
                match handle.updater() {
                    Ok(updater) => {
                        match updater.check().await {
                            Ok(update) => {
                                if let Some(update) = update {
                                    println!("[Updater] 新版本: {}", update.version);
                                    // 有更新，下载并安装
                                    if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                                        eprintln!("[Updater] 下载失败: {}", e);
                                    } else {
                                        // 更新完成，重启应用
                                        handle.restart();
                                    }
                                } else {
                                    println!("[Updater] 已是最新版本");
                                }
                            }
                            Err(e) => {
                                eprintln!("[Updater] 检查更新失败: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[Updater] 获取 updater 失败: {}", e);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
