const chokidar = require("chokidar");

const { createStaticServer } = require("./staticServer");
const { createReloadBroadcaster } = require("./reloadBroadcaster");
const { compile } = require("./compile");
const { DOKTORW, PORT } = require("./paths");

async function startServer(doktorFile) {
    const httpServer = createStaticServer(DOKTORW, PORT);
    const { broadcastReload } = createReloadBroadcaster(httpServer);

    httpServer.listen(PORT, () => {
        console.log(`doktors running at http://localhost:${PORT}`);
        console.log(`Watching ${doktorFile} for changes...`);
    });

    // Initial compile so the first page load has something to render.
    try {
        await compile(doktorFile);
    } catch(error) {
        console.error(`Initial compile failed: ${error.message}`);
    }

    const watcher = chokidar.watch(doktorFile, {
        // A simple debouncing for file saving, to prevent rapid compiling.
        awaitWriteFinish: {
            stabilityThreshold: 100,
            pollInterval: 20,
        },
    });

    watcher.on("change", async () => {
        console.log(`${doktorFile} changed, recompiling...`);

        try {
            await compile(doktorFile);
            console.log("Recompiled. Reloading browser...");
            broadcastReload();
        } catch (error) {
            console.error(`Compile failed: ${error.message}`);
        }
    });
}

module.exports = { startServer };