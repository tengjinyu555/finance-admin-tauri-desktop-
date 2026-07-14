const { app, BrowserWindow, globalShortcut } = require('electron')
const path = require('path')

let mainWindow

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 1000,
    minHeight: 700,
    title: '业财一体化管理',
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  })

  // 开发模式加载 localhost，生产模式加载 dist 文件
  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:5173')
    mainWindow.webContents.openDevTools()
  } else {
    mainWindow.loadFile(path.join(__dirname, '../dist/index.html'))
  }

  mainWindow.on('closed', () => { mainWindow = null })
}

app.whenReady().then(() => {
  createWindow()

  // F12 打开开发者工具
  globalShortcut.register('F12', () => {
    if (mainWindow) {
      mainWindow.webContents.toggleDevTools()
    }
  })
})

app.on('window-all-closed', () => {
  globalShortcut.unregisterAll()
  app.quit()
})

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) createWindow()
})
