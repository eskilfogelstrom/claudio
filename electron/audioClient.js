const net = require('net');
const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');
const crypto = require("crypto");
const { app, dialog } = require('electron');


let isPackaged;

if (
    process.mainModule &&
    process.mainModule.filename.indexOf('app.asar') !== -1
) {
    isPackaged = true;
} else if (process.argv.filter(a => a.indexOf('app.asar') !== -1).length > 0) {
    isPackaged = true;
}

const serialize = (type, payload = {}) => {
    const obj = {
        type,
        ...payload
    };
    const json = JSON.stringify(obj);
    const buf = Buffer.alloc(256);
    buf.write(json);

    return buf;
};

const deserialize = data => JSON.parse(data.toString());

class Socket {
    constructor(socket) {
        this._socket = socket;
    }

    config = () => {
        const promise = new Promise((resolve, reject) => {
            const cb = data => {
                const msg = deserialize(data);
                resolve(msg);
            };
            
            this._socket.once('data', cb);

            this._socket.once('error', err => {
                reject(err);
            });
        });

        const msg = serialize('config');
        this._socket.write(msg);

        return promise;
    }

    connect = config => {
        const promise = new Promise((resolve, reject) => {
            const cb = data => {
                const msg = deserialize(data);
                resolve(msg);
            };
            
            this._socket.once('data', cb);

            this._socket.once('error', err => {
                reject(err);
            });
        });

        const msg = serialize('connect', {config});
        this._socket.write(msg);

        return promise;
    }
    
    stream = async (remote, mode, config) => {
        const msg = serialize('stream', {
            remote_addr: remote,
            mode,
            config
        });
        this._socket.write(msg);
    }
}

const createSocket = socketPath => {
    const promise = new Promise((resolve, reject) => {
        setTimeout(() => {
            const socket = net.createConnection(socketPath);

            socket.once('error', () => {
                reject();
            });

            socket.once('close', () => {
                reject();
            });

            socket.on('connect', () => resolve(new Socket(socket)));
        }, 500);
    });

    return promise;
}

const init = async () => {

    const TMP_FOLDER = '/tmp/com.claudio.App';
    
    if (!fs.existsSync(TMP_FOLDER)) {
        fs.mkdirSync(TMP_FOLDER);
    }

    const socketName = crypto.randomBytes(4).toString('hex') + '.sock';
    const socketPath = path.join(TMP_FOLDER, socketName);
    
    const binaryPath = isPackaged 
        ? path.join(path.dirname(app.getAppPath()), '..', './Resources', './bin')
        : path.join(app.getAppPath(), './src-rust', './target/debug');

    const binaryExecPath  = path.resolve(path.join(binaryPath, './p2p_audio'));

    const p = spawn(binaryExecPath, [socketPath]);

    p.stdout.on('data', data => console.log(data.toString()));
    p.stderr.on('data', data => console.log(data.toString()));

    app.on('before-quit', function () {
        p.kill();
    });

    let socket;
    let tries = 0;
    while (true) {
        try {
            socket = await createSocket(socketPath);
            break;
        } catch(e) {
            if (tries < 5) {
                tries += 1;
                continue;
            }

            throw e;
        }
    }

    return socket;
}

module.exports = init;