const fetch = require('node-fetch');

const perform = (path, options) => {
    return fetch('http://192.168.10.123:4000' + path, {
        headers: {
            'Content-Type': 'application/json',
        },
        ...options
    })
    .then(res => {
        if (res.status < 300) {
            return res.json();
        } else {
            throw Error(res.status);
        }
    });
}

const get = path => perform(path, {
    method: 'GET'
});

const post = (path, body) => perform(path, {
    method: 'POST',
    body: JSON.stringify(body)
});

module.exports = {
    get,
    post
};