import React from 'react';

const Send = () => {
    const [connection, setConnection] = React.useState();
    const [error, setError] = React.useState();
    const [id, setId] = React.useState('');

    const connect = () => {
        window.electron.ipcRenderer
            .invoke('send', 'syn', id)
            .then(connection => setConnection(connection))
            .catch(e => setError(e));
    };

    const stream = () => {
        window.electron.ipcRenderer
            .invoke('send', 'stream', connection);
    }

    React.useEffect(() => {
        window.electron.ipcRenderer.on('send', (payload) => {
            setConnection(payload);
        });
    }, []);

    return (
        <div>
            {connection ? (
                <div>
                    ID: {connection.id}<br/>
                    State: {connection.state}<br/>
                    Local: {connection.send_address}<br/>
                    Remote: {connection.return_address}<br/>
                    {connection.state === 'SynAcked' && (
                        <button onClick={stream}>Stream</button>
                    )}
                </div>
            ): (
                <>
                    <input onChange={e => setId(e.target.value)} value={id}/>
                    <button onClick={connect}>Connect</button>
                </>
            )}
        </div>
    )
};

export default Send;