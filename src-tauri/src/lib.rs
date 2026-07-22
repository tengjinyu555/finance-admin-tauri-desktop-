
use std::fs::OpenOptions;
use std::io::Write;
use tauri::{Emitter, Listener};

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

            // 注册：收到重启指令后执行更新脚本
            handle.listen("update-restart", move |_| {
                log_to_file("收到重启指令");
                if let Ok(current_exe) = std::env::current_exe() {
                    let exe_dir = current_exe.parent().unwrap();
                    let new_path = exe_dir.join("update_new.exe");

                    log_to_file(&format!("当前exe: {}", current_exe.display()));
                    log_to_file(&format!("新文件: {}", new_path.display()));

                    // 等待几秒让文件句柄释放
                    log_to_file("等待3秒...");
                    std::thread::sleep(std::time::Duration::from_secs(3));

                    // 用 cmd 执行操作，用引号包裹路径处理中文
                    let current = current_exe.display().to_string();
                    let new_file = new_path.display().to_string();
                    let name = current_exe.file_name().unwrap().to_str().unwrap();

                    let cmd = format!(
                        r#"cmd /c "timeout /t 2 /nobreak >nul & del /f /q "{}" & ren "{}" "{}" & start "" "{}""#,
                        current, new_file, name, current
                    );
                    log_to_file(&format!("执行cmd: {}", cmd));

                    match std::process::Command::new("cmd")
                        .args(["/c", &cmd])
                        .spawn() {
                            Ok(mut child) => {
                                log_to_file("更新命令已执行，等待完成...");
                                let _ = child.wait();
                                log_to_file("cmd 命令执行完成");
                            }
                            Err(e) => log_to_file(&format!("执行失败: {}", e)),
                        }

                    log_to_file("退出当前应用...");
                    std::process::exit(0);
                }
            });

            // 注册：收到下载指令后开始下载
            let handle_clone = handle.clone();
            handle.listen("update-start-download", move |_| {
                log_to_file("收到下载指令，开始下载...");
                let handle = handle_clone.clone();
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        use tauri_plugin_updater::UpdaterExt;
                        match handle.updater() {
                            Ok(updater) => {
                                match updater.check().await {
                                    Ok(update) => {
                                        if let Some(update) = update {
                                            let download_url = update.download_url.to_string();
                                            log_to_file(&format!("下载地址: {}", download_url));

                                            let client = reqwest::Client::new();
                                            match client.get(&download_url).send().await {
                                                Ok(resp) => {
                                                    let total_size = resp.content_length().unwrap_or(0);
                                                    log_to_file(&format!("文件大小: {} bytes", total_size));

                                                    let mut downloaded: u64 = 0;
                                                    let mut bytes_all = Vec::new();
                                                    let mut stream = resp.bytes_stream();
                                                    use futures_util::StreamExt;

                                                    while let Some(chunk) = stream.next().await {
                                                        match chunk {
                                                            Ok(data) => {
                                                                downloaded += data.len() as u64;
                                                                bytes_all.extend_from_slice(&data);
                                                                let _ = handle.emit("update-progress", serde_json::json!({
                                                                    "loaded": downloaded,
                                                                    "total": total_size
                                                                }));
                                                            }
                                                            Err(e) => {
                                                                log_to_file(&format!("下载流错误: {}", e));
                                                                let _ = handle.emit("update-error", format!("下载失败: {}", e));
                                                                return;
                                                            }
                                                        }
                                                    }

                                                    log_to_file(&format!("下载完成，大小: {} bytes", bytes_all.len()));

                                                    if let Ok(current_exe) = std::env::current_exe() {
                                                        let exe_dir = current_exe.parent().unwrap();
                                                        let new_path = exe_dir.join("update_new.exe");
                                                        let bat_path = exe_dir.join("update.bat");

                                                        if let Ok(mut file) = std::fs::File::create(&new_path) {
                                                            use std::io::Write;
                                                            let _ = file.write_all(&bytes_all);
                                                            log_to_file("新文件已保存");

                                                            let current_str = current_exe.display().to_string();
                                                            let new_str = new_path.display().to_string();
                                                            let current_name = current_exe.file_name().unwrap().to_str().unwrap();

                                                            // 使用简单的 BAT 脚本
                                                            let bat_content = format!(
                                                                "@echo off\r\n\
                                                                 echo 等待3秒...\r\n\
                                                                 timeout /t 3 /nobreak > nul\r\n\
                                                                 echo 删除旧版本...\r\n\
                                                                 del /f /q \"{}\" 2>nul\r\n\
                                                                 echo 重命名新版本...\r\n\
                                                                 ren \"{}\" \"{}\" 2>nul\r\n\
                                                                 echo 启动新版本...\r\n\
                                                                 start \"\" \"{}\"\r\n\
                                                                 echo 清理...\r\n\
                                                                 del /f /q \"{}\" 2>nul\r\n\
                                                                 del /f /q \"{}\" 2>nul\r\n\
                                                                 del /f /q \"%~f0\" 2>nul",
                                                                current_str,
                                                                new_str,
                                                                current_name,
                                                                current_str,
                                                                new_str,
                                                                bat_path.display()
                                                            );
                                                            if let Ok(mut bat_file) = std::fs::File::create(&bat_path) {
                                                                let _ = bat_file.write_all(bat_content.as_bytes());
                                                                log_to_file("BAT脚本已创建");
                                                                let _ = handle.emit("update-complete", "");
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    log_to_file(&format!("下载请求失败: {}", e));
                                                    let _ = handle.emit("update-error", format!("下载失败: {}", e));
                                                }
                                            }
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
            });

            // 启动时检查更新（只通知，不下载）
            std::thread::spawn(move || {
                log_to_file("检查更新线程启动");
                std::thread::sleep(std::time::Duration::from_secs(3));
                log_to_file("开始检查更新");
                log_to_file(&format!("当前版本: {}", env!("CARGO_PKG_VERSION")));

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
                                        let _ = handle.emit("update-available", update.version.clone());
                                        log_to_file("已通知前端发现新版本");
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
