
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
                    let ps_path = exe_dir.join("update.ps1");

                    // 读取并记录脚本内容
                    if let Ok(content) = std::fs::read_to_string(&ps_path) {
                        log_to_file(&format!("脚本内容: {}", content));
                    }

                    log_to_file(&format!("脚本路径: {}", ps_path.display()));

                    // 启动 PowerShell 脚本
                    match std::process::Command::new("powershell")
                        .args(["-ExecutionPolicy", "Bypass", "-File", &ps_path.to_string_lossy()])
                        .spawn() {
                            Ok(_) => {
                                log_to_file("PowerShell 脚本已启动");
                            }
                            Err(e) => {
                                log_to_file(&format!("启动 PowerShell 失败: {}", e));
                            }
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
                                                        let ps_path = exe_dir.join("update.ps1");

                                                        if let Ok(mut file) = std::fs::File::create(&new_path) {
                                                            use std::io::Write;
                                                            let _ = file.write_all(&bytes_all);
                                                            log_to_file("新文件已保存");

                                                            let current_str = current_exe.display().to_string();
                                                            let new_str = new_path.display().to_string();
                                                            let ps_str = ps_path.display().to_string();
                                                            let current_name = current_exe.file_name().unwrap().to_str().unwrap();

                                                            let ps_content = format!(
                                                                "Start-Transcript -Path '{}' -Force\n\
                                                                 Write-Host '等待3秒...'\n\
                                                                 Start-Sleep -Seconds 3\n\
                                                                 Write-Host '删除旧版本: {}'\n\
                                                                 Remove-Item -Path '{}' -Force -ErrorAction SilentlyContinue\n\
                                                                 Write-Host '重命名新版本'\n\
                                                                 Rename-Item -Path '{}' -NewName '{}' -Force\n\
                                                                 Write-Host '启动新版本'\n\
                                                                 Start-Process -FilePath '{}' -Wait\n\
                                                                 Write-Host '清理'\n\
                                                                 Remove-Item -Path '{}' -Force -ErrorAction SilentlyContinue\n\
                                                                 Remove-Item -Path '{}' -Force -ErrorAction SilentlyContinue\n\
                                                                 Stop-Transcript",
                                                                exe_dir.join("update.log").display(),
                                                                current_str,
                                                                current_str,
                                                                new_str,
                                                                current_name,
                                                                current_str,
                                                                new_str,
                                                                ps_str
                                                            );
                                                            if let Ok(mut ps_file) = std::fs::File::create(&ps_path) {
                                                                let _ = ps_file.write_all(ps_content.as_bytes());
                                                                log_to_file("更新脚本已创建");
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
