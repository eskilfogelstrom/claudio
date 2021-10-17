import React, { useEffect } from 'react';

const Config = ({configs, config, setConfig }) => {
    const handleHost = e => {
        setConfig({
            ...config,
            host: e.target.value,
        });
    };

    const handleInputDevice = e => {
        setConfig({
            ...config,
            inputDevice: e.target.value,
        });
    };

    const handleOutputDevice = e => {
        setConfig({
            ...config,
            outputDevice: e.target.value,
        });
    };

    const handleSampleRate = e => {
        setConfig({
            ...config,
            sampleRate: Number(e.target.value),
        });
    };

    const handleBufferSize = e => {
        setConfig({
            ...config,
            bufferSize: Number(e.target.value)
        });
    };

    const handleStereo = e => {
        setConfig({
            ...config,
            stereo: e.target.checked
        });
    };

    const handleInputChannel = e => {
        setConfig({
            ...config,
            inputChannel: Number(e.target.value)
        });
    }

    const handleOutputChannel = e => {
        setConfig({
            ...config,
            outputChannel: Number(e.target.value)
        });
    }

    useEffect(() => {
        setConfig({
            ...config,
            host: Object.keys(configs)[0],
        });
    }, [configs]);

    useEffect(() => {
        if (config.host) {
            const devices = Object.keys(configs[config.host]);
            setConfig({
                ...config,
                inputDevice: devices[0],
                outputDevice: devices[0]
            });
        }
    }, [config.host]);

    useEffect(() => {
        if (config.inputDevice && config.outputDevice) {
            const sampleRates = configs[config.host][config.inputDevice].input.sample_rates.filter(a => {
                return configs[config.host][config.outputDevice].output.sample_rates.some(b => a === b);
            });
            const bufferSizes = [64, 128, 256, 512].filter(s => {
                return (
                    s >= inputDevice.input.buffer_size.min && s <= inputDevice.input.buffer_size.max &&
                    s >= outputDevice.output.buffer_size.min && s <= outputDevice.output.buffer_size.max
                );
            });
            setConfig({
                ...config,
                sampleRate: sampleRates[0],
                bufferSize: bufferSizes[0],
                inputChannel: 0,
                outputChannel: 0,
                stereo: false
            });
        }
    }, [config.inputDevice, config.outputDevice]);

    useEffect(() => {
        if (config.stereo) {
            if (config.inputChannel % 2 !== 0) {
                setConfig({
                    ...config,
                    inputChannel: config.inputChannel - 1
                });
            }

            if (config.outputChannel % 2 !== 0) {
                setConfig({
                    ...config,
                    outputChannel: config.outputChannel - 1
                });
            }
        }
    }, [config.stereo, config.inputChannel, config.outputChannel]);


    const hosts = Object.keys(configs);
    const host = configs[config.host];

    if (!host) {
        return null;
    }

    const devices = Object.keys(host);
    const inputDevice = host[config.inputDevice];
    const outputDevice = host[config.outputDevice];

    if (!inputDevice || !outputDevice) {
        return null;
    }

    const sampleRates = inputDevice.input.sample_rates.filter(a => {
        return outputDevice.output.sample_rates.some(b => a === b);
    });

    const bufferSizes = [64, 128, 256, 512].filter(s => {
        return (
            s >= inputDevice.input.buffer_size.min && s <= inputDevice.input.buffer_size.max &&
            s >= outputDevice.output.buffer_size.min && s <= outputDevice.output.buffer_size.max
        );
    });
    
    const inputChannels = [...Array(inputDevice.input.channels ? inputDevice.input.channels : 0).keys()]
        .filter(channel => {
            if (config.stereo) {
                return channel % 2 === 0
            } else {
                return true;
            }
        });
    const outputChannels = [...Array(outputDevice.output.channels ? outputDevice.output.channels : 0).keys()]
        .filter(channel => {
            if (config.stereo) {
                return channel % 2 === 0
            } else {
                return true;
            }
        });

    return (
        <>
            <div>
                <label>Host</label>
                <select onChange={handleHost} value={config.host}>
                    {hosts.map(host => (
                        <option key={host}>{host}</option>
                    ))}
                </select>
            </div>
            <div>
                <label>Input device</label>
                <select onChange={handleInputDevice} value={config.inputDevice}>
                    {devices.map(input => (
                        <option key={input} value={input} disabled={!host[input].input.channels}>
                            {input}
                            ({host[input].input.channels} in, {host[input].output.channels} out)
                        </option>
                    ))}
                </select>
            </div>
            <div>
                <label>Output device</label>
                <select onChange={handleOutputDevice} value={config.outputDevice}>
                    {devices.map(output => (
                        <option key={output} value={output} disabled={!host[output].output.channels}>
                            {output}
                            ({host[output].input.channels} in, {host[output].output.channels} out)
                        </option>
                    ))}
                </select>
            </div>
            <div>
                <label>Sample Rate</label>
                <select onChange={handleSampleRate} value={config.sampleRate}>
                    {sampleRates.map(sampleRate => (
                        <option key={sampleRate} value={sampleRate}>{sampleRate}</option>
                    ))}
                </select>
            </div>
            <div>
                <label>Buffer size</label>
                <select onChange={handleBufferSize} value={config.bufferSize}>
                    {bufferSizes.map(bufferSize => (
                        <option key={bufferSize} value={bufferSize}>{bufferSize}</option>
                    ))}
                </select>
            </div>
            <div>
                <label>Stereo</label>
                <input type="checkbox" onChange={handleStereo} value={config.stereo} />
            </div>
            <div>
                <label>Input channel</label>
                <select onChange={handleInputChannel} value={config.inputChannel}>
                    {inputChannels.map(channel => (
                        <option key={channel} value={channel}>{channel + 1}{config.stereo && `/${channel + 2}`}</option>
                    ))}
                </select>
            </div>
            <div>
                <label>Output channel</label>
                <select onChange={handleOutputChannel} value={config.outputChannel}>
                    {outputChannels.map(channel => (
                        <option key={channel} value={channel}>{channel + 1}{config.stereo && `/${channel + 2}`}</option>
                    ))}
                </select>
            </div>
        </>
    );
};

export default Config;