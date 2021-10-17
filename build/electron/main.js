const { app, BrowserWindow } = require('electron');
const path = require('path');
const url = require('url');

const handlers = require('./handlers');

let mainWindow;

function createWindow () {
  const startUrl = process.env.ELECTRON_START_URL || url.format({
    pathname: path.join(__dirname, '../index.html'),
    protocol: 'file:',
    slashes: true,
  });

  mainWindow = new BrowserWindow({
    width: 400,
    height: 600,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
    },
  });

  handlers(mainWindow)
    .then(() => {
      mainWindow.loadURL(startUrl);
      mainWindow.on('closed', function () {
        mainWindow = null;
      });
    })
    .catch(() => {
      app.quit();
    });
}
app.on('ready', createWindow);
app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
app.on('activate', function () {
  if (mainWindow === null) {
    createWindow();
  }
});