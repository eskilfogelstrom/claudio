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

const createSocket = socketPath => {
    const promise = new Promise((resolve, reject) => {
        setTimeout(() => {
            const socket = net.createConnection(socketPath);

            const connect = () => {
                const promise = new Promise((resolve, reject) => {
                    const cb = data => {
                        const msg = deserialize(data);
                        resolve(msg);
                    };
                    
                    socket.once('data', cb);

                    socket.once('error', err => {
                        reject(err);
                    });
                });
        
                const msg = serialize('connect');
                socket.write(msg);

                return promise;
            }
            
            const stream = async (remote, mode) => {
                const msg = serialize('stream', {
                    remote_addr: remote,
                    mode,
                });
                socket.write(msg);

                return promise;
            }

            socket.once('error', () => {
                reject();
            });

            socket.once('close', () => {
                reject();
            });

            socket.on('connect', () => resolve({
                connect,
                stream
            }));
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

    p.stdout.on('data', data => dialog.showErrorBox('Stdout', data.toString()));
    p.stderr.on('data', data => dialog.showErrorBox('Stderr', data.toString()));

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