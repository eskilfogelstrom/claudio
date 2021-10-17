const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const WebSocket = require('ws');

const client = require('./client');
const AudioClient = require('./audioClient');

module.exports = async (mainWindow) => {

  const audioClient = await AudioClient();

  const returnHandlers = {
    listen: async () => {
      const connection = await client.post('/api/listen/', {});
      
      mainWindow.webContents.send('return', connection);
  
      const ws = new WebSocket(`ws://192.168.10.123:4000/?id=${connection.id}`);
  
      ws.on('close', (reason) => {
        console.log(reason);
      });
  
      ws.on('message', message => {
        const connection = JSON.parse(message);
        mainWindow.webContents.send('return', connection);
      });
    },
    syn: async id => {
      try {
        const msg = await audioClient.connect();

        const connection = await client.post('/api/syn/', {
          id,
          send_address: msg.address
        });

        mainWindow.webContents.send('send', connection);

        const ws = new WebSocket(`ws://192.168.10.123:4000/?id=${connection.id}`);
    
        ws.on('close', (reason) => {
          console.log(reason);
        });
    
        ws.on('message', message => {
          const connection = JSON.parse(message);
          mainWindow.webContents.send('send', connection);
        });
      } catch(e) {
        dialog.showErrorBox('Error', e);
      }
    },
    ack: async id => {
      const msg = await audioClient.connect();
      const connection = await client.post('/api/ack/', {
        id,
        return_address: msg.address
      });

      audioClient.stream(connection.send_address, 'Return');

      mainWindow.webContents.send('return', connection);
    },
    stream: async connection => {
      audioClient.stream(connection.return_address, 'Send');
    }
  };

  ipcMain.on('return', (event, eventName, payload) => {
    returnHandlers[eventName](payload);
  });

  ipcMain.on('send', (event, eventName, payload) => {
    returnHandlers[eventName](payload);
  });
};