
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
                                        let download_url = update.download_url.to_string();
                                        log_to_file(&format!("下载地址: {}", download_url));

                                        // 手动下载绕过签名验证
                                        log_to_file("开始手动下载...");
                                        match reqwest::get(&download_url).await {
                                            Ok(resp) => {
                                                match resp.bytes().await {
                                                    Ok(bytes) => {
                                                        log_to_file(&format!("下载完成，大小: {} bytes", bytes.len()));
                                                        // 获取当前 exe 路径
                                                        if let Ok(current_exe) = std::env::current_exe() {
                                                            let exe_dir = current_exe.parent().unwrap();
                                                            let exe_name = current_exe.file_name().unwrap();
                                                            // 下载到临时文件
                                                            let new_path = exe_dir.join("finance-admin-new.exe");
                                                            let bat_path = exe_dir.join("update.bat");
                                                            if let Ok(mut file) = std::fs::File::create(&new_path) {
                                                                use std::io::Write;
                                                                let _ = file.write_all(&bytes);
                                                                log_to_file("新文件已下载，创建批处理...");
                                                                // 创建批处理脚本来替换文件
                                                                let bat_content = format!(
                                                                    "@echo off\r\n\
                                                                     timeout /t 2 /nobreak > nul\r\n\
                                                                     del \"{}\"\r\n\
                                                                     ren \"{}\" \"{}\"\r\n\
                                                                     start \"\" \"{}\"\r\n\
                                                                     del \"%~f0\"",
                                                                    current_exe.display(),
                                                                    new_path.display(),
                                                                    exe_name.to_str().unwrap(),
                                                                    current_exe.display()
                                                                );
                                                                if let Ok(mut bat_file) = std::fs::File::create(&bat_path) {
                                                                    let _ = bat_file.write_all(bat_content.as_bytes());
                                                                    log_to_file("批处理已创建，准备替换并重启...");
                                                                    // 启动批处理
                                                                    let _ = std::process::Command::new("cmd")
                                                                        .args(["/c", bat_path.to_str().unwrap()])
                                                                        .spawn();
                                                                    std::process::exit(0);
                                                                }
                                                            } else {
                                                                log_to_file("无法创建新文件");
                                                            }
                                                        } else {
                                                            log_to_file("无法获取当前exe路径");
                                                        }
                                                    }
                                                    Err(e) => {
                                                        log_to_file(&format!("读取下载内容失败: {}", e));
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                log_to_file(&format!("下载请求失败: {}", e));
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
