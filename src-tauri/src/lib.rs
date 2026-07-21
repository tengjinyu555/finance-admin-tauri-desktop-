
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Manager;

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
    log_to_file("构建 Builder...");
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            log_to_file("setup 开始");
            let handle = app.handle().clone();

            // 使用 std::thread 避免 async 问题
            std::thread::spawn(move || {
                log_to_file("检查更新线程启动");
                std::thread::sleep(std::time::Duration::from_secs(2));
                log_to_file("开始检查更新");
                log_to_file(&format!("当前版本: {}", env!("CARGO_PKG_VERSION")));

                // 创建 tokio runtime 来执行 async 操作
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    use tauri_plugin_updater::UpdaterExt;
                    match handle.updater() {
                        Ok(updater) => {
                            log_to_file("updater 获取成功，开始检查...");
                            match updater.check().await {
                                Ok(update) => {
                                    log_to_file("检查完成");
                                    if let Some(update) = update {
                                        log_to_file(&format!("发现新版本: {}", update.version));
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
            });

            log_to_file("setup 完成");
            Ok(())
        })
        .build(tauri::generate_context!());

    match result {
        Ok(app) => {
            log_to_file("构建成功，准备运行");
            app.run(|_app_handle, _event| {});
        }
        Err(e) => {
            log_to_file(&format!("构建失败: {}", e));
        }
    }
}
