const { WebSocketServer } = require("ws");

function createReloadBroadcaster(httpServer) {
    const wss = new WebSocketServer({ server: httpServer });

    function broadcastReload() {
        for(const client of wss.clients) {
            if(client.readyState === client.OPEN) client.send("reload");
        }
    }

    return { broadcastReload };
}

module.exports = { createReloadBroadcaster };