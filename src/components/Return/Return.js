import React from 'react';

import Config from '../Config';

import './Return.css';

const Return = () => {
    const [connection, setConnection] = React.useState();
    const [configs, setConfigs] = React.useState();
    const [config, setConfig] = React.useState({});

    const connect = () => {
        window.electron.ipcRenderer
            .invoke('return', 'listen', {
                ...config,
            })
            .then(connection => setConnection(connection))
            .catch(e => console.log(e));
    };

    const ack = () => {
        window.electron.ipcRenderer
            .invoke('return', 'ack', connection.id)
            .then(connection => setConnection(connection));;
    }

    React.useEffect(() => {
        window.electron.ipcRenderer.on('return', (payload) => {
            setConnection(payload);
        });

        window.electron.ipcRenderer.invoke('return', 'config').then(config => {
            setConfigs(config);
        });
    }, []);

    if (!configs) {
        return null;
    }

    return (
        <div className="return">
            {connection ? (
                <div>
                    ID: {connection.id}<br/>
                    State: {connection.state}<br/>
                    Return: {connection.return_address}<br/>
                    Send: {connection.send_address}
                    {connection.state === 'SynSent' && (
                        <button onClick={ack}>Ack</button>
                    )}
                </div>
            ) : (
                <>
                    <Config configs={configs} config={config} setConfig={setConfig}/>
                    <button onClick={connect}>Connect</button>
                </>
            )}
        </div>
    )
};

export default Return;