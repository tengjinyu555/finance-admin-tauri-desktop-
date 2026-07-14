const { app, BrowserWindow, globalShortcut, dialog, shell } = require('electron')
const path = require('path')
const http = require('http')
const https = require('https')
const fs = require('fs')
const { spawn } = require('child_process')

// 设置应用名称（任务栏显示）
app.name = '业财一体化管理'

// 处理更新替换逻辑
if (process.argv.includes('--update-replace')) {
  const oldExePath = process.argv[process.argv.indexOf('--update-replace') + 1]
  const currentExePath = process.execPath

  // 延迟执行替换
  setTimeout(() => {
    try {
      // 删除旧exe
      if (fs.existsSync(oldExePath)) {
        fs.unlinkSync(oldExePath)
      }

      // 把自己移动到旧位置
      fs.renameSync(currentExePath, oldExePath)

      // 清理临时目录
      const tempDir = path.dirname(currentExePath)
      if (tempDir.includes('temp_update')) {
        fs.rmSync(tempDir, { recursive: true, force: true })
      }

      // 启动新位置的exe
      spawn(oldExePath, [], { detached: true, stdio: 'ignore' }).unref()

      // 退出
      app.quit()
      process.exit(0)
    } catch (e) {
      console.error('更新替换失败:', e)
      // 替换失败，正常启动
    }
  }, 1000)
}

let mainWindow
// 从package.json读取版本号，改版本只需改package.json的version字段
const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8'))
const CURRENT_VERSION = packageJson.version

function checkVersion() {
  return new Promise((resolve) => {
    const req = http.get('http://finance.52youran.top/api/version', (res) => {
      let data = ''
      res.on('data', chunk => { data += chunk })
      res.on('end', () => {
        try {
          const result = JSON.parse(data)
          resolve(result)
        } catch (e) {
          resolve(null)
        }
      })
    })
    req.on('error', () => { resolve(null) })
    req.setTimeout(5000, () => { req.destroy(); resolve(null) })
    req.end()
  })
}

async function showUpdateDialog(downloadUrl) {
  const url = downloadUrl || 'http://finance.52youran.top/download/业财一体化管理.exe'
  const result = await dialog.showMessageBox(mainWindow, {
    type: 'info',
    title: '发现新版本',
    message: '发现新版本，是否立即更新？',
    buttons: ['暂不更新', '立即更新'],
    defaultId: 1
  })
  if (result.response === 1) {
    await downloadAndUpdate(url)
  }
}

// 下载并自动更新
async function downloadAndUpdate(url) {
  const currentExePath = process.execPath
  const tempDir = path.join(path.dirname(currentExePath), 'temp_update')
  const tempExePath = path.join(tempDir, '业财一体化管理_new.exe')

  // 创建临时目录
  if (!fs.existsSync(tempDir)) {
    fs.mkdirSync(tempDir, { recursive: true })
  }

  // 显示下载进度
  const progressWindow = new BrowserWindow({
    width: 400,
    height: 150,
    frame: false,
    resizable: false,
    alwaysOnTop: true,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  })

  progressWindow.loadURL(`data:text/html;charset=utf-8,${encodeURIComponent(`
    <!DOCTYPE html>
    <html>
    <head>
      <style>
        body { font-family: 'Microsoft YaHei', sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: #f5f5f5; }
        .container { text-align: center; }
        .progress-text { margin-bottom: 10px; color: #333; }
        .progress-bar { width: 300px; height: 20px; background: #e0e0e0; border-radius: 10px; overflow: hidden; }
        .progress-fill { width: 0%; height: 100%; background: linear-gradient(90deg, #409eff, #67c23a); transition: width 0.3s; }
      </style>
    </head>
    <body>
      <div class="container">
        <div class="progress-text" id="text">正在下载更新...</div>
        <div class="progress-bar"><div class="progress-fill" id="progress"></div></div>
      </div>
      <script>
        window.downloadProgress = (percent) => {
          document.getElementById('progress').style.width = percent + '%';
          document.getElementById('text').textContent = '正在下载更新... ' + percent + '%';
        }
      </script>
    </body>
    </html>
  `)}`)

  try {
    // 下载文件
    await downloadFile(url, tempExePath, (percent) => {
      if (progressWindow && !progressWindow.isDestroyed()) {
        progressWindow.webContents.executeJavaScript(`downloadProgress(${percent})`)
      }
    })

    // 关闭进度窗口
    if (progressWindow && !progressWindow.isDestroyed()) {
      progressWindow.close()
    }

    // 启动新版，传递当前exe路径作为参数
    const child = spawn(tempExePath, ['--update-replace', currentExePath], {
      detached: true,
      stdio: 'ignore'
    })
    child.unref()

    // 退出当前应用
    app.quit()

  } catch (e) {
    if (progressWindow && !progressWindow.isDestroyed()) {
      progressWindow.close()
    }
    dialog.showErrorBox('更新失败', '下载更新失败：' + e.message)
  }
}

// 下载文件
function downloadFile(url, destPath, onProgress) {
  return new Promise((resolve, reject) => {
    const client = url.startsWith('https') ? https : http

    client.get(url, (response) => {
      if (response.statusCode === 301 || response.statusCode === 302) {
        downloadFile(response.headers.location, destPath, onProgress)
          .then(resolve)
          .catch(reject)
        return
      }

      if (response.statusCode !== 200) {
        reject(new Error('下载失败，状态码：' + response.statusCode))
        return
      }

      const totalSize = parseInt(response.headers['content-length'], 10) || 0
      let downloadedSize = 0

      const file = fs.createWriteStream(destPath)

      response.on('data', (chunk) => {
        downloadedSize += chunk.length
        if (totalSize > 0 && onProgress) {
          const percent = Math.round((downloadedSize / totalSize) * 100)
          onProgress(percent)
        }
      })

      response.pipe(file)

      file.on('finish', () => {
        file.close()
        resolve()
      })

      file.on('error', (err) => {
        fs.unlink(destPath, () => {})
        reject(err)
      })
    }).on('error', (err) => {
      reject(err)
    })
  })
}

function checkDevServer() {
  return new Promise((resolve) => {
    const req = http.get('http://localhost:5173', (res) => {
      resolve(true)
    })
    req.on('error', () => {
      resolve(false)
    })
    req.end()
  })
}

async function createWindow() {
  // 根据环境选择图标路径
  const isDev = !app.isPackaged
  const iconPath = isDev
    ? path.join(__dirname, '..', 'build', 'icon.png')
    : path.join(process.resourcesPath, 'build', 'icon.png')

  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 1000,
    minHeight: 700,
    title: '业财一体化管理',
    icon: iconPath,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  })

  // 检测开发服务器是否运行
  const devServerRunning = await checkDevServer()

  if (devServerRunning) {
    mainWindow.loadURL('http://localhost:5173')
    mainWindow.webContents.openDevTools()
  } else {
    // 生产模式：加载打包后的index.html
    const distPath = path.join(__dirname, '..', 'dist', 'index.html')
    console.log('Loading:', distPath)
    mainWindow.loadFile(distPath)
  }

  mainWindow.on('closed', () => { mainWindow = null })
}

app.whenReady().then(async () => {
  await createWindow()

  // F12 打开开发者工具
  globalShortcut.register('F12', () => {
    if (mainWindow) {
      mainWindow.webContents.toggleDevTools()
    }
  })

  // 检查版本更新
  const versionInfo = await checkVersion()
  if (versionInfo && versionInfo.version !== CURRENT_VERSION) {
    showUpdateDialog(versionInfo.downloadUrl)
  }
})

app.on('window-all-closed', () => {
  globalShortcut.unregisterAll()
  app.quit()
})

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) createWindow()
})
