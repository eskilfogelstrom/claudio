const { ipcRenderer, contextBridge } = require('electron');

contextBridge.exposeInMainWorld('electron', {
    ipcRenderer: {
      invoke(channel, ...args) {
        return ipcRenderer.invoke(channel, ...args);
      },
      send(channel, ...args) {
        ipcRenderer.send(channel, ...args);
      },
      on(channel, func) {
        const validChannels = ['return', 'send'];
        if (validChannels.includes(channel)) {
          // Deliberately strip event as it includes `sender`
          ipcRenderer.on(channel, (event, ...args) => func(...args));
        }
      },
      once(channel, func) {
        const validChannels = ['return', 'send'];
        if (validChannels.includes(channel)) {
          // Deliberately strip event as it includes `sender`
          ipcRenderer.once(channel, (event, ...args) => func(...args));
        }
      },
    },
  });