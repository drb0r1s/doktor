const chokidar = require("chokidar");
const path = require("path");

const { createStaticServer } = require("./staticServer");
const { createReloadBroadcaster } = require("./reloadBroadcaster");
const { compile } = require("./compile");
const { DOKTORW, PORT } = require("./paths");
const { PREFIX } = require("../data/prefix");

async function startServer(doktorFile) {
    const httpServer = createStaticServer(DOKTORW, PORT);
    const { broadcastReload } = createReloadBroadcaster(httpServer);

    httpServer.listen(PORT, () => {
        console.log(`${PREFIX} Running at http://localhost:${PORT}...`);
        console.log(`${PREFIX} Watching ${path.basename(doktorFile)} for changes...`);
    });

    // Initial compile so the first page load has something to render.
    try {
        await compile(doktorFile);
    } catch(error) {
        console.error(`${PREFIX} Initial compile failed: ${error.message}`);
    }

    const watcher = chokidar.watch(doktorFile, {
        // A simple debouncing for file saving, to prevent rapid compiling.
        awaitWriteFinish: {
            stabilityThreshold: 100,
            pollInterval: 20,
        },
    });

    watcher.on("change", async () => {
        console.log(`${PREFIX} ${path.basename(doktorFile)} has been updated, recompiling...`);

        try {
            await compile(doktorFile);
            console.log(`${PREFIX} Recompiled. Reloading browser...`);

            broadcastReload();
        } catch (error) {
            console.error(`${PREFIX} Compile failed: ${error.message}`);
        }
    });
}

module.exports = { startServer };