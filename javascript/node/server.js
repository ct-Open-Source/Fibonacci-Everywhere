var http = require('http');

const fib = (n) => {
    let result = [];
    let prev = 0n;
    let sum = 1n;
    for (let i = 0; i < n; ++i) {
        result.push(sum);
        [prev, sum] = [sum, prev + sum];
    }
    return result;
};

BigInt.prototype.toJSON = function () { return this.toString(); };

const server = http.createServer(function (req, res) {
    const found = req.url.match(new RegExp("/fibonacci/(\\d+)"));
    if (found) {
        const n = parseInt(found[1]);
        const result = fib(n);
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.write(JSON.stringify({
            'n': n,
            'seq': result,
        }));
        res.end();
    }
    else {
        res.end('ERROR: invalid request');
    }
});

const PORT = 17711;
server.listen(PORT);
console.log(`Web server listening on port ${PORT} ...`)
