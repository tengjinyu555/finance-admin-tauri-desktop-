const { app, BrowserWindow, globalShortcut, dialog, shell } = require('electron')
const path = require('path')
const http = require('http')
const https = require('https')
const fs = require('fs')

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
  const url = downloadUrl || 'http://finance.52youran.top/download/财税管理平台.exe'
  const result = await dialog.showMessageBox(mainWindow, {
    type: 'info',
    title: '发现新版本',
    message: '发现新版本，是否前往下载？',
    buttons: ['暂不更新', '前往下载'],
    defaultId: 1
  })
  if (result.response === 1) {
    shell.openExternal(url)
  }
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
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 1000,
    minHeight: 700,
    title: '财税管理平台',
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  })

  // 检测开发服务器是否运行
  const isDev = await checkDevServer()

  if (isDev) {
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
