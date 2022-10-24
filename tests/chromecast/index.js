const ChromecastAPI = require('chromecast-api')
const statik = require('node-static');
const http = require('http');

const file = new statik.Server('./public');
const client = new ChromecastAPI()

const deviceName = 'Chromecast-c9fc7011da51bc559f43750eb01749e3._googlecast._tcp.local';

client.on('device', function (device) {
    var mediaURL = 'http://192.168.1.42:8080/park.mp4';

    if (device.name !== deviceName) return;

    device.play(mediaURL, (err) => {
        if (err) throw err;
    })
})

http.createServer(function (request, response) {
    request.addListener('end', function () {
        file.serve(request, response);
    }).resume();
}).listen(8080);
