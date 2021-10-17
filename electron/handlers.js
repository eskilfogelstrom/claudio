const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const WebSocket = require('ws');

const client = require('./client');
const AudioClient = require('./audioClient');

module.exports = async (mainWindow) => {

  const audioClient = await AudioClient();

  const returnHandlers = {
    config: () => audioClient.config(),
    listen: async config => {
      const connection = await client.post('/api/listen/', config);
      
      const ws = new WebSocket(`ws://192.168.10.123:4000/?id=${connection.id}`);
  
      ws.on('close', (reason) => {
        console.log(reason);
      });
  
      ws.on('message', message => {
        const connection = JSON.parse(message);
        mainWindow.webContents.send('return', connection);
      });

      return connection;
    },
    syn: async id => {
      let connection = await client.post('/api/config/', {
        id,
      });

      const msg = await audioClient.connect(connection.config);

      if (!msg.is_valid) {
        throw new Error('Config not supported');
      }

      connection = await client.post('/api/syn/', {
        id,
        send_address: msg.address
      });

      const ws = new WebSocket(`ws://192.168.10.123:4000/?id=${connection.id}`);
  
      ws.on('close', (reason) => {
        console.log(reason);
      });
  
      ws.on('message', message => {
        const connection = JSON.parse(message);
        mainWindow.webContents.send('send', connection);
      });

      return connection;
    },
    ack: async id => {
      const msg = await audioClient.connect();
      const connection = await client.post('/api/ack/', {
        id,
        return_address: msg.address
      });

      audioClient.stream(connection.send_address, 'Return', connection.config);

      return connection;
    },
    stream: async connection => {
      audioClient.stream(connection.return_address, 'Send', connection.config);
    }
  };

  ipcMain.handle('return', (event, eventName, ...args) => {
    return returnHandlers[eventName](...args);
  });

  ipcMain.handle('send', async (event, eventName, ...args) => {
    return returnHandlers[eventName](...args);
  });
};